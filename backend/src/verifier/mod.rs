pub mod api;
pub mod db;

use log::{debug, info};
use std::{io::Write, path::PathBuf, str::FromStr};
use tokio::spawn;

use clap::Parser;
use concordium_rust_sdk::{
    id::types::AttributeTag,
    smart_contracts::common::EntrypointName,
    types::{
        smart_contracts::{ContractName, OwnedReceiveName},
        ContractAddress, Energy, WalletAccount,
    },
    v2::BlockIdentifier,
};
use poem::{
    listener::TcpListener,
    middleware::{Cors, CorsEndpoint},
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;

use self::{
    api::{Api, StatementType},
    db::Db,
};

#[derive(Parser, Debug, Clone)]
pub struct VerifierApiConfig {
    #[clap(env)]
    pub concordium_node_uri: String,
    #[clap(env)]
    pub verifier_web_server_addr: String,
    #[clap(env)]
    pub mongodb_uri: String,
    #[clap(env)]
    pub identity_registry: String,
    #[clap(env)]
    pub agent_wallet_path: PathBuf,
    #[clap(env, default_value = "init_rwa_identity_registry")]
    pub rwa_identity_registry_contract_name: String,
    #[clap(env, default_value = "registerIdentity")]
    pub rwa_identity_registry_register_identity_fn_name: String,
    #[clap(env, default_value = "30000")]
    pub register_identity_max_energy: String,
}
pub async fn run_verifier_api_server(config: VerifierApiConfig) -> anyhow::Result<()> {
    debug!("Starting Verifier API Server with config: {:?}", config);

    let routes = create_server_routes(config.to_owned()).await?;
    let web_server_addr = config.verifier_web_server_addr.clone();
    let server_handle =
        spawn(async move { Server::new(TcpListener::bind(web_server_addr)).run(routes).await });
    info!("Listening for web requests at {}", config.verifier_web_server_addr);
    server_handle.await??;
    info!("Shutting Down...");
    Ok(())
}

async fn create_server_routes(config: VerifierApiConfig) -> anyhow::Result<CorsEndpoint<Route>> {
    let api_service = create_service(config).await?;
    let ui = api_service.swagger_ui();
    let routes = Route::new().nest("/", api_service).nest("/ui", ui).with(Cors::new());

    Ok(routes)
}

async fn create_service(
    config: VerifierApiConfig,
) -> Result<OpenApiService<Api, ()>, anyhow::Error> {
    let mongo_client = mongodb::Client::with_uri_str(&config.mongodb_uri)
        .await
        .unwrap_or_else(|_| panic!("Failed to connect to MongoDB at url {}", config.mongodb_uri));
    let mut concordium_client = concordium_rust_sdk::v2::Client::new(
        concordium_rust_sdk::v2::Endpoint::from_str(&config.concordium_node_uri)?,
    )
    .await
    .unwrap_or_else(|_| panic!("Failed to connect to Concordium Node at url {}", config.concordium_node_uri));
    let global_context =
        concordium_client.get_cryptographic_parameters(BlockIdentifier::LastFinal).await?.response;
    let agent_wallet = WalletAccount::from_json_file(config.agent_wallet_path)?;
    let identity_registry = ContractAddress::from_str(&config.identity_registry)?;
    let register_identity_receive_name = OwnedReceiveName::construct(
        ContractName::new(&config.rwa_identity_registry_contract_name)?,
        EntrypointName::new(&config.rwa_identity_registry_register_identity_fn_name)?,
    )?;
    let statement: StatementType = StatementType::new()
        // Should be older than 18
        .older_than(18)
        // unwrap here because value is hardcoded
        .unwrap()
        // reveal nationality : needed for compliant transfers
        .reveal_attribute(AttributeTag(5));

    let api_service = OpenApiService::new(
        Api {
            statement,
            identity_registry,
            db: Db {
                client: mongo_client.to_owned(),
                identity_registry,
                agent_address: agent_wallet.address,
            },
            concordium_client,
            agent_wallet,
            register_identity_receive_name,
            global_context,
            max_energy: Energy::from_str(&config.register_identity_max_energy)?,
        },
        "RWA Contracts API",
        "1.0.0",
    );
    Ok(api_service)
}

#[derive(Parser, Debug, Clone)]
pub struct VerifierApiSwaggerConfig {
    #[clap(env, default_value = "verifier-api-specs.json")]
    pub output: String,
    #[clap(env, default_value = "http://node.testnet.concordium.com:20000")]
    pub concordium_node_uri: String,
    #[clap(env, default_value = "0.0.0.0:3001")]
    pub verifier_web_server_addr: String,
    #[clap(env, default_value = "mongodb://root:example@localhost:27017")]
    pub mongodb_uri: String,
    /// Identity Registry Contract String
    #[clap(env, default_value = "<7762,0>")]
    pub identity_registry: String,
    #[clap(env, default_value = "init_rwa_identity_registry")]
    pub rwa_identity_registry_contract_name: String,
    #[clap(env, default_value = "registerIdentity")]
    pub rwa_identity_registry_register_identity_fn_name: String,
    /// Identity Registry Agent Wallet Path
    #[clap(env, default_value = "agent_wallet.export")]
    pub agent_wallet_path: PathBuf,
    /// Max energy to use for register identity
    #[clap(env, default_value = "30000")]
    pub register_identity_max_energy: String,
}

impl From<VerifierApiSwaggerConfig> for VerifierApiConfig {
    fn from(config: VerifierApiSwaggerConfig) -> Self {
        Self {
            concordium_node_uri: config.concordium_node_uri,
            verifier_web_server_addr: config.verifier_web_server_addr,
            mongodb_uri: config.mongodb_uri,
            identity_registry: config.identity_registry,
            rwa_identity_registry_contract_name: config.rwa_identity_registry_contract_name,
            rwa_identity_registry_register_identity_fn_name: config
                .rwa_identity_registry_register_identity_fn_name,
            agent_wallet_path: config.agent_wallet_path,
            register_identity_max_energy: config.register_identity_max_energy,
        }
    }
}

pub async fn generate_verifier_api_frontend_client(
    config: VerifierApiSwaggerConfig,
) -> anyhow::Result<()> {
    let api_service = create_service(config.to_owned().into()).await?;
    let spec_json = api_service.spec();
    let mut file = std::fs::File::create(config.output)?;
    file.write_all(spec_json.as_bytes())?;
    Ok(())
}
