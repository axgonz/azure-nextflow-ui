mod routes;
mod app; 
mod services;

use routes::{
    count::*,
};
use app::{
    variables::*,
    state::*,
    secrets::*,
};
use services::{
    az_storage::*,
};

use az_app_identity::*;
use actix_web::{
    web::Data,
    App, 
    HttpServer
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr: String = match std::env::var("ACTIXWEB_SERVE_ADDRESS") {
        Ok(value) => value,
        Err(_) => "127.0.0.1".to_string()
    };
    let port: u16 = match std::env::var("ACTIXWEB_SERVE_PORT") {
        Ok(value) => value.parse().unwrap(),
        Err(_) => "3030".parse().unwrap()
    };

    let app_identity = AppIdentity::new();
    let mut app_variables = AppVariables::new();
    let mut app_secrets = AppSecrets::new();

    /* 
        Uncomment when ready to initialize variables and secrets. 
        See respective crate README.md for more details. 
    */ 
    // AppVariables::init(&mut app_variables);
    // AppSecrets::init(&mut app_secrets, &app_variables.azure_keyvault_name, app_identity.clone()).await;
    
    let app_state = AppState {
        identity: app_identity,
        variables: app_variables,
        secrets: app_secrets,
        counter: Mutex::new(0),
    };
    println!("{:#?}", &app_state.variables);
    let data = Data::new(app_state);

    println!("\nListening on http://{}:{}\n", addr, port);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(get_api)
            .service(get_api_add)
            .service(get_api_sub)
    })
    .bind((addr, port))?
    .run()
    .await
}