use clicrate::cli::Cli;
use cling::{Cling, ClingFinished};


#[tokio::main]
async fn main() -> ClingFinished<Cli>{
    Cling::parse_and_run().await
}
