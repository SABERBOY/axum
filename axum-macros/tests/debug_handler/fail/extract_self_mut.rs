use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};
use axum_macros::debug_handler;

struct A;

#[async_trait]
impl FromRequest for A {
    type Rejection = ();

    async fn from_request(_req: &mut RequestParts) -> Result<Self, Self::Rejection> {
        unimplemented!()
    }
}

impl A {
    #[debug_handler]
    async fn handler(&mut self) {}
}

fn main() {}
