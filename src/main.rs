use anyhow::Context;
use clap::Parser;

use tokio::task::JoinSet;
use upstream::{
    Args, DnsResolver, FileTlsServerConfigLoader, Host, LoadBalancer, TcpHost,
    TlsServerConfigProvider, Upstream,
    config::{Config, DnsResolverConfig, HostConfigKind, RuntimeConfig},
};

fn main() -> anyhow::Result<()> {
    let filter = std::env::var("RUST_LOG").unwrap_or(String::from("upstream=info"));

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let args = Args::parse();

    let config = Config::from_yaml_file(&args.config_file)?;

    let future = async move {
        let mut set = JoinSet::new();

        let resolver = match config.dns_resolver {
            DnsResolverConfig::Hickory => DnsResolver::hickory()?,
            DnsResolverConfig::Tokio => DnsResolver::tokio(),
        };

        for mut host in config.hosts {
            let host = {
                let load_balancer = {
                    match host.upstreams.len() {
                        0 => {
                            return Err(anyhow::anyhow!(format!(
                                "Upstream vector for host {host:?} is empty"
                            )));
                        }
                        1 => {
                            let upstream = host.upstreams.remove(0);

                            let upstream = Upstream::from_config(upstream, resolver.clone());

                            LoadBalancer::identity(upstream)
                        }
                        _ => LoadBalancer::static_fifo(Box::leak(
                            host.upstreams
                                .into_iter()
                                .map(|upstream| Upstream::from_config(upstream, resolver.clone()))
                                .collect::<Vec<_>>()
                                .into_boxed_slice(),
                        )),
                    }
                };

                match host.kind {
                    HostConfigKind::Tcp => Host::tcp(TcpHost::plain(host.addr, load_balancer)),
                    HostConfigKind::Tls { certs } => {
                        let loader = FileTlsServerConfigLoader::new(certs.certs, certs.key);

                        let provider = match certs.watch {
                            None => TlsServerConfigProvider::static_file(loader).await?,
                            Some(watch) => {
                                TlsServerConfigProvider::watch_file(loader, watch).await?
                            }
                        };

                        Host::tcp(TcpHost::tls(host.addr, load_balancer, provider))
                    }
                }
            };

            set.spawn(host.run());
        }

        tokio::select! {
            results = set.join_all() => {
                for result in results {
                    result?;
                }
            }
            _ = shutdown_signal() => {

            }
        }

        Ok::<(), anyhow::Error>(())
    };

    let mut runtime = match config.runtime {
        RuntimeConfig::MultiThread => tokio::runtime::Builder::new_multi_thread(),
        RuntimeConfig::CurrentThread => tokio::runtime::Builder::new_current_thread(),
    };

    runtime
        .enable_io()
        .enable_time()
        .build()
        .context("Failed to build runtime")?
        .block_on(future)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");

        tracing::info!("CTRL+C received");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM signal handler")
            .recv()
            .await;

        tracing::info!("SIGTERM received");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutting down");
}
