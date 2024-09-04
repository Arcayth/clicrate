use clap::Parser;
use cling::prelude::*;
use serde::Deserialize;
use cli_table::{print_stdout, Table, WithTitle};

#[derive(Run, Parser, Debug, Clone)]
pub struct Cli {
    #[command(subcommand)]
    pub command:Commands,
}

#[derive(Run, Parser, Debug, Clone)]
pub enum Commands{
    /// Search for crate
    Search(SearchOpts),
}

#[derive(Run,Collect, Parser, Debug, Clone)]
#[cling(run="search")]
pub struct SearchOpts{
    pub query: String,

    #[arg(short, long, required = false, default_value_t = 5)]
    /// Number of results to be displayed
    pub limit:u32,

    #[arg(short, long, required = false)]
    /// Search for the exact name (If not provided search by number of downloads)
    pub exact:bool,
}

#[derive(Deserialize, Debug, Table)]
struct Crate{
    #[table(title = "Name")]
    name: String,
    #[table(title = "Description")]
    description:String,
    #[table(title = "Downloads")]
    downloads: u64,
    #[table(title = "Version")]
    max_version: String,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    crates: Option<Vec<Crate>>
}

pub async fn search(SearchOpts { query, limit ,exact}: &SearchOpts) {
    let url = if !exact {
        format!("https://crates.io/api/v1/crates?q={}&sort=downloads&per_page={}", query, limit)
    }else {
        format!("https://crates.io/api/v1/crates?q={}&sort=relevance&per_page={}", query, limit)
    };

    if let Some(result) = search_api_handler(url).await{
        print_stdout(result.with_title()).unwrap();
    }
}

async fn search_api_handler(url: String) -> Option<Vec<Crate>>{
    let client = reqwest::Client::new();
    let res = client.get(&url)
        .header("User-Agent", "clicrate")
        .send().await.unwrap();
    let text = res.text().await.unwrap();

    let api_response: ApiResponse = serde_json::from_str(&text).unwrap();
    let crate_list = api_response.crates; 
    crate_list
}
