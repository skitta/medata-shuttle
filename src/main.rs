use models::DbPool;
use service::{AppRouter, AppState};

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] postgres: DbPool,
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // let (stripe_key, stripe_sub_price, mailgun_key, mailgun_url, domain) = grab_secrets(secrets);
    let domain = grab_secrets(secrets);

    let state = AppState::new(postgres, domain).await;

    let router = AppRouter::router(state);

    Ok(router.into())
}

fn grab_secrets(secrets: shuttle_runtime::SecretStore) -> String {
    // let stripe_key = secrets
    //     .get("STRIPE_KEY")
    //     .unwrap_or_else(|| "None".to_string());

    // let stripe_sub_price = secrets
    //     .get("STRIPE_SUB_PRICE")
    //     .unwrap_or_else(|| "None".to_string());

    // let mailgun_key = secrets
    //     .get("MAILGUN_KEY")
    //     .unwrap_or_else(|| "None".to_string());

    // let mailgun_url = secrets
    //     .get("MAILGUN_URL")
    //     .unwrap_or_else(|| "None".to_string());

    let domain = secrets
        .get("DOMAIN_URL")
        .unwrap_or_else(|| "None".to_string());

    // (
    //     stripe_key,
    //     stripe_sub_price,
    //     mailgun_key,
    //     mailgun_url,
    //     domain,
    // )
    domain
}
