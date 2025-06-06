use aries_vcx_wallet::wallet::askar::{
    askar_wallet_config::AskarWalletConfig, key_method::KeyMethod, AskarWallet,
};
use log::info;
use url::Url;
use mediator::aries_agent::AgentBuilder;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    load_dot_env();
    setup_logging();
    info!("Starting up mediator! ⚙️⚙️");
    let endpoint_root = std::env::var("ENDPOINT_ROOT").unwrap_or("127.0.0.1:8005".into());
    info!("Mediator endpoint root address: {}", endpoint_root);
    // default wallet config for a dummy in-memory wallet
    let default_wallet_config = AskarWalletConfig::new(
        "sqlite://:memory:",
        KeyMethod::Unprotected,
        "",
        &Uuid::new_v4().to_string(),
    );
    let wallet_config_json = std::env::var("INDY_WALLET_CONFIG");
    let wallet_config = wallet_config_json
        .ok()
        .map_or(default_wallet_config, |json| {
            serde_json::from_str(&json).unwrap()
        });
    info!("Wallet Config: {:?}", wallet_config);
    let mut agent = AgentBuilder::<AskarWallet>::new_from_wallet_config(wallet_config)
        .await
        .unwrap();
    let credo_compatible: bool = std::env::var("CREDO_COMPATIBLE")
        .unwrap_or("false".into())
        .parse()
        .unwrap();
    agent
        .init_service(
            vec![],
            get_service_endpoint(endpoint_root.clone()),
            credo_compatible
        )
        .await
        .unwrap();
    let app_router = mediator::http_routes::build_router(agent).await;
    info!("Starting server");
    let listener = tokio::net::TcpListener::bind(&endpoint_root).await.unwrap();
    axum::serve(listener, app_router.into_make_service())
        .await
        .unwrap();
}

fn setup_logging() {
    let env = env_logger::Env::default().default_filter_or("info");
    env_logger::init_from_env(env);
}

fn load_dot_env() {
    let _ = dotenvy::dotenv();
}

fn get_service_endpoint(endpoint_root: String) -> Url {
    let endpoint: String = std::env::var("MEDIATOR_ADDRESS")
        .unwrap_or(format!("http://{endpoint_root}"));
    format!("{endpoint}/didcomm").parse().unwrap()
}
