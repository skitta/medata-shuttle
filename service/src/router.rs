use axum::{
    http::{self},
    routing::{get, post, put},
    // middleware::{self},
    Router,
};
use http::header::{ACCEPT, AUTHORIZATION, ORIGIN};
use http::HeaderValue;
use http::Method;
use tower_http::cors::CorsLayer;

use crate::controller::{
    add::add_patient,
    auth::{login, logout, register},
    detail::document_detail,
    list::{document_list, group_list},
    update::update_patient,
};
use crate::AppState;

pub fn create_api_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT])
        .allow_origin(state.domain.parse::<HeaderValue>().unwrap());

    let auth_router = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", get(logout));

    let enrollgroup_router = Router::new().route("/", get(group_list));

    let document_router = Router::new()
        .route("/", get(document_list))
        .route("/:id", get(document_detail));

    let patient_router = Router::new()
        .route("/add", post(add_patient))
        .route("/:id", put(update_patient));

    // let kawasaki_router = Router::new().route("/add", post(add_kawasaki));

    Router::new()
        .nest("/enrollGroup", enrollgroup_router)
        .nest("/document", document_router)
        .nest("/patient", patient_router)
        // .nest("/kawasaki", kawasaki_router)
        // .layer(middleware::from_fn_with_state(
        //     state.clone(),
        //     validate_session,
        // ))
        .nest("/auth", auth_router)
        .with_state(state)
        .layer(cors)
}
