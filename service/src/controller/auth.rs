use crate::AppState;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar, SameSite};
use time::Duration;

use models::auth::{
    session,
    user::{self, LoginUser, RegisterUser},
};

pub async fn register(
    State(state): State<AppState>,
    Json(newuser): Json<RegisterUser>,
) -> impl IntoResponse {
    match user::create(&state.postgres, newuser).await {
        Ok(_) => (StatusCode::CREATED, "Account created!".to_string()).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            format!("Something went wrong: {e}"),
        )
            .into_response(),
    }
}

pub async fn login(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    Json(login): Json<LoginUser>,
) -> Result<(PrivateCookieJar, StatusCode), impl IntoResponse> {
    match user::verify(&state.postgres, login).await {
        Ok(res) => {
            let session_id = session::create(&state.postgres, &res)
                .await
                .expect("Couldn't create session :(");

            let cookie = Cookie::build(("medata", session_id))
                .secure(!cfg!(debug_assertions)) // Only send the cookie over HTTPS in production
                .same_site(SameSite::Strict)
                .http_only(true)
                .path("/")
                .max_age(Duration::WEEK)
                .build();

            Ok((jar.add(cookie), StatusCode::OK))
        }

        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string()).into_response()),
    }
}

pub async fn logout(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
) -> Result<PrivateCookieJar, StatusCode> {
    let Some(cookie) = jar.get("sessionid").map(|cookie| cookie.value().to_owned()) else {
        return Ok(jar);
    };

    match session::delete(&state.postgres, cookie).await {
        Ok(_) => Ok(jar.remove(Cookie::from("medata"))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[allow(dead_code)]
pub async fn validate_session(
    jar: PrivateCookieJar,
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> (PrivateCookieJar, Response) {
    let Some(cookie) = jar.get("medata").map(|cookie| cookie.value().to_owned()) else {
        println!("Couldn't find a cookie in the jar");
        return (
            jar,
            (StatusCode::FORBIDDEN, "Forbidden!".to_string()).into_response(),
        );
    };

    match session::find(&state.postgres, cookie).await {
        Ok(_) => (jar, next.run(request).await),
        Err(_) => (
            jar,
            (StatusCode::FORBIDDEN, "Forbidden!".to_string()).into_response(),
        ),
    }
}
