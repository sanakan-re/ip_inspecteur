extern crate serde_json;
// use serde::Deserialize;
use serde_json::Value as JsonValue;
// use std::collections::HashMap;
use reqwest::header::HeaderMap;
use std::env;
use std::process;

use ip_inspecteur::Params;
use ip_inspecteur::run;

// #[derive(Deserialize, Debug)]
// struct Ip_intel {
//     ip_sus: String
// }


fn main() {

    let args: Vec<String> = env::args().collect();
    
    let params = Params::new(&args).unwrap_or_else( |err | {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("IP adress: {}\n", params.ip);

    if let Err(e) = run(params) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

#[tokio::main]
async fn virust_req() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "31616d3c6fe801289c3db2730cf07cb35f682abf21a00c79136af88bdb3dd797";

    let mut headers = HeaderMap::new();
    headers.insert("x-apikey", format!("{}", api_key).parse().unwrap());

    let client = reqwest::Client::new();
    let resp = client
        .get("https://www.virustotal.com/api/v3/ip_addresses/10.10.10.10")
        .headers(headers)
        .send()
        //.json::<HashMap<String, String>>()
        .await?;

    // println!("{:#?}", resp);

    //let resp_json = resp.json::<HashMap<String, JsonValue>>().await?;
    let resp_json = resp.json::<JsonValue>().await?;

    println!("{:#?}", resp_json["data"]["attributes"]["last_analysis_stats"]);

    Ok(())
}


