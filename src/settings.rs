use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub upstreams: UpstreamsConfig,
    pub proxy: ProxyConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub listen_addr: String,
}

#[derive(Debug, Deserialize)]
pub struct UpstreamsConfig {
    pub addresses: Vec<String>,
    pub health_check_frequency: u64, // in seconds
}

#[derive(Debug, Deserialize)]
pub struct ProxyConfig {
    pub upstream_identifier: String,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::File::with_name("config/config"))
            .build()?;
        cfg.try_deserialize()
    }
}
