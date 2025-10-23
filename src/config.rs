use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    time::Duration,
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub runtime: RuntimeConfig,
    pub dns_resolver: DnsResolverConfig,
    pub hosts: Vec<HostConfig>,
}

impl Config {
    fn from_yaml(yaml: &[u8]) -> anyhow::Result<Self> {
        let config = serde_yaml::from_slice(yaml)?;

        Ok(config)
    }

    pub fn from_yaml_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();

        let yaml = std::fs::read(path)?;

        Self::from_yaml(&yaml)
    }
}

#[derive(Debug, Deserialize)]
pub enum RuntimeConfig {
    MultiThread,
    CurrentThread,
}

#[derive(Debug, Deserialize)]
pub enum DnsResolverConfig {
    Hickory,
    Tokio,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum HostConfigKind {
    Tcp,
    Tls { certs: HostConfigCerts },
}

#[derive(Debug, Deserialize)]
pub struct HostConfig {
    pub addr: SocketAddr,
    pub upstreams: Vec<UpstreamConfig>,
    #[serde(flatten)]
    pub kind: HostConfigKind,
}

#[derive(Debug, Deserialize)]
pub struct HostConfigCerts {
    pub certs: PathBuf,
    pub key: PathBuf,
    pub watch: Option<Watch>,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(tag = "kind")]
pub enum Watch {
    Debounce {
        #[serde(with = "humantime_serde")]
        duration: Duration,
    },
    Poll {
        #[serde(with = "humantime_serde")]
        duration: Duration,
    },
}

#[derive(Debug, Deserialize)]
pub struct UpstreamConfig {
    pub domain: String,
    pub port: u16,
}
