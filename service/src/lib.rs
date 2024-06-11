use axum::extract::FromRef;
use axum::Router;
use axum_extra::extract::cookie::Key;
use models::DbPool;
use tower_http::services::{ServeDir, ServeFile};

mod controller;
mod router;

#[derive(Clone)]
pub struct AppState {
    pub postgres: DbPool,
    // pub stripe_key: String,
    // pub stripe_sub_price: String,
    // pub mailgun_key: String,
    // pub mailgun_url: String,
    pub domain: String,
    pub key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

impl AppState {
    pub async fn new(pg: DbPool, domain_url: String) -> Self {
        models::init_db(&pg).await;

        AppState {
            postgres: pg,
            domain: domain_url,
            key: Key::generate(),
        }
    }
}

pub struct AppRouter;

impl AppRouter {
    pub fn router(state: AppState) -> Router {
        let router = Router::new()
            .nest("/api", router::create_api_router(state))
            .nest_service(
                "/",
                ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
            );
        return router;
    }
}
