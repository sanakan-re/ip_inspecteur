extern crate serde_json;
// use serde::Deserialize;
use serde_json::Value as JsonValue;
// use std::collections::HashMap;
use reqwest::header::HeaderMap;
// use std::env;
use std::process;
use clap::Parser;
use ip_inspecteur::Args;
use ip_inspecteur::run;

// #[derive(Deserialize, Debug)]
// struct Ip_intel {
//     ip_sus: String
// }


fn main() {

    // let args: Vec<String> = env::args().collect();
    let args = Args::parse();

    // virust_req(&args.ip);
    
//     let params = Params::new(&args).unwrap_or_else( |err | {
//         println!("Problem parsing arguments: {}", err);
//         process::exit(1);
//     });

    // println!("IP adress: {}\n", args.ip);

    // if let Err(e) = run(args) {
    //     println!("Application error: {}", e);
    //     process::exit(1);
    // }
    if let Err(e) = virust_req(&args.ip) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

#[tokio::main]
async fn virust_req(ip: &String) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "31616d3c6fe801289c3db2730cf07cb35f682abf21a00c79136af88bdb3dd797";
    let vt_adress = String::from("https://www.virustotal.com/api/v3/ip_addresses/"); 

    let _vt_adress_i = vt_adress + &ip;

    let mut headers = HeaderMap::new();
    headers.insert("x-apikey", format!("{}", api_key).parse().unwrap());

    let client = reqwest::Client::new();
    let resp = client
        .get(_vt_adress_i)
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


