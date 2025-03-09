mod settings;
use settings::Settings;

use async_trait::async_trait;
use std::{sync::Arc, time::Duration};
use pingora::prelude::*;

// LB now carries the load balancer and the identifier from configuration.
pub struct LB {
    lb: Arc<LoadBalancer<RoundRobin>>,
    identifier: String,
}

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self.lb.select(b"", 256).unwrap();
        println!("upstream peer is: {:?}", upstream);
        let peer = Box::new(HttpPeer::new(upstream, true, self.identifier.clone()));
        Ok(peer)
    }
}

fn main() {
    // Load configuration from config/config.toml.
    let settings = Settings::new().expect("Failed to load configuration");

    // Initialize the server.
    let mut my_server = Server::new(None).unwrap();
    my_server.bootstrap();

    // Create the load balancer using addresses from the configuration.
    let mut upstreams = LoadBalancer::try_from_iter(
        settings.upstreams.addresses.iter().map(|addr| addr.as_str())
    )
    .unwrap();

    let hc = TcpHealthCheck::new();
    upstreams.set_health_check(hc);
    upstreams.health_check_frequency = Some(Duration::from_secs(settings.upstreams.health_check_frequency));
    let background = background_service("health check", upstreams);
    let upstreams = background.task();

    // Create the LB instance using the configured upstream_identifier.
    let lb = LB {
        lb: upstreams,
        identifier: settings.proxy.upstream_identifier.clone(),
    };
    let mut proxy_service = http_proxy_service(&my_server.configuration, lb);
    proxy_service.add_tcp(&settings.server.listen_addr);

    my_server.add_service(proxy_service);
    my_server.add_service(background);
    my_server.run_forever();
}
