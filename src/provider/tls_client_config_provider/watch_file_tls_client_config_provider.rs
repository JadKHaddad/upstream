use std::sync::Arc;

use anyhow::Context;
use arc_swap::ArcSwap;
use futures::StreamExt;
use rustls::ClientConfig;

use crate::{Watcher, config::Watch, loader::FileTlsClientConfigLoader};

#[derive(Clone)]
pub struct WatchFileTlsClientConfigProvider {
    config: Arc<ArcSwap<ClientConfig>>,
}

/// TODO: Duplicate of watch_file_tls_server_config_provider.rs, can we abstract over server/client?
impl WatchFileTlsClientConfigProvider {
    pub async fn new(loader: FileTlsClientConfigLoader, watch: Watch) -> anyhow::Result<Self> {
        let config = loader.load().await.context("Failed to load config")?;
        let config = Arc::new(ArcSwap::new(Arc::new(config)));

        let watch_config = config.clone();

        let (watch_tx, watch_rx) = futures::channel::oneshot::channel();

        tokio::spawn(async move {
            let (tx, mut rx) = futures::channel::mpsc::unbounded();

            let mut watcher = match watch {
                Watch::Debounce { duration } => Watcher::debounce(duration, tx),
                Watch::Poll { duration } => Watcher::poll(duration, tx),
            }?;

            let paths = loader.paths();

            watcher.watch_many(&paths)?;

            tracing::info!(?paths, "Watching for changes");

            let _ = watch_tx.send(Ok::<(), anyhow::Error>(()));

            while let Some(res) = rx.next().await {
                match res {
                    Ok(event) => {
                        tracing::debug!(?event, "Change detected");

                        tracing::info!(?paths, "Reloading config");

                        match loader.load().await {
                            Ok(config) => {
                                watch_config.store(Arc::new(config));

                                tracing::info!(?paths, "Config loaded successfuly");
                            }
                            Err(err) => {
                                tracing::error!(%err, ?paths, "Error loading config")
                            }
                        }
                    }

                    Err(err) => tracing::error!(%err, ?paths, "Error watching for changes"),
                }
            }

            tracing::warn!(?paths, "Watch task terminated");

            Ok::<(), anyhow::Error>(())
        });

        watch_rx
            .await
            .expect("Channel can not be closed before sending a value")
            .context("Failed to install watcher")?;

        Ok(Self { config })
    }

    pub fn get_client_config(&self) -> Arc<ClientConfig> {
        self.config.load().clone()
    }
}
