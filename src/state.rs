use crate::PgPool;

pub struct  GlobalState {
    pub db: PgPool,
}