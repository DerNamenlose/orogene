use async_std::sync::{Arc, Mutex};
use async_trait::async_trait;
use futures::io::AsyncRead;
use oro_client::OroClient;

use super::PackageFetcher;

use crate::error::{Error, Internal, Result};
use crate::package::{Manifest, Package, PackageRequest, Packument};

pub struct RegistryFetcher {
    client: Arc<Mutex<OroClient>>,
    packument: Option<Packument>,
    _manifest: Option<Manifest>,
}

impl RegistryFetcher {
    pub fn new(client: Arc<Mutex<OroClient>>) -> Self {
        Self {
            client,
            packument: None,
            _manifest: None,
        }
    }
}

#[async_trait]
impl PackageFetcher for RegistryFetcher {
    async fn manifest(&mut self, _pkg: &Package) -> Result<Manifest> {
        todo!()
    }

    async fn packument(&mut self, pkg: &PackageRequest) -> Result<Packument> {
        if self.packument.is_none() {
            let client = self.client.lock().await;
            self.packument = Some(
                client
                    .get(pkg.name().await?)
                    .await
                    .with_context(|| "Failed to get packument.".into())?
                    .body_json::<Packument>()
                    .await
                    .map_err(|e| Error::MiscError(e.to_string()))?,
            );
        }
        // Safe unwrap. We literally JUST assigned it :P
        Ok(self.packument.clone().unwrap())
    }

    async fn tarball(&mut self, _arg: &Package) -> Result<Box<dyn AsyncRead + Send + Sync>> {
        unimplemented!()
    }
}
