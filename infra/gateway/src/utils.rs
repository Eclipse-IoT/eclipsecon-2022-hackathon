use async_trait::async_trait;
use bluer::mesh::{network::Network, node::Node};
use dbus::Path;
use std::time::Duration;

#[async_trait]
pub trait AttachRetry {
    async fn attach_retry(
        &self,
        mut retries: usize,
        delay: Duration,
        path: Path<'_>,
        token: &str,
    ) -> bluer::Result<Node>;
}

#[async_trait]
impl AttachRetry for Network {
    async fn attach_retry(
        &self,
        mut retries: usize,
        delay: Duration,
        path: Path<'_>,
        token: &str,
    ) -> bluer::Result<Node> {
        loop {
            match self.attach(path.clone(), token).await {
                Ok(node) => break Ok(node),
                Err(err) if retries == 0 => break Err(err),
                Err(err) => {
                    log::warn!("Failed to attach (re-tries left: {retries}): {err}");
                    retries -= 1;
                    tokio::time::sleep(delay).await;
                }
            };
        }
    }
}
