use crate::{Crypto, PgPool};

pub struct GlobalState {
    pub db: PgPool,
    pub crypto: Crypto,
}
