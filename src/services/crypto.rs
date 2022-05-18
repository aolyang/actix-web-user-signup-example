use argonautica::Hasher;
use color_eyre::Result;
use eyre::eyre;
use futures::compat::Future01CompatExt;
use std::sync::Arc;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct Crypto {
    pub key: Arc<String>,
}

impl Crypto {
    #[instrument(self, pwd)]
    pub async fn hash_pwd(&self, pwd: String) -> Result<String> {
        Hasher::default()
            .with_secret_key(&*self.key)
            .with_password(pwd)
            .hash_non_blocking()
            .compat()
            .await
            .map_err(|err| eyre!("Hashing error: {:?}", err))
    }
}
