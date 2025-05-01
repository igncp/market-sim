// TODO: Remove this after a while
#![allow(dead_code)]

use cli::{Action, Cli};
use server::run_server;

mod cli;
mod core;
mod logger;
mod server;
mod simulation;
mod storage;
mod storage_interface;

#[actix_web::main]
async fn main() {
    let action = Cli::parse().await;

    match action {
        Action::StartServer(opts) => {
            run_server(opts).await.unwrap();
        }
    }
}
