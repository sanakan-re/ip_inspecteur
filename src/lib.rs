use std::fs;

extern crate serde_json;
// use serde::Deserialize;
use serde_json::Value as JsonValue;
// use std::collections::HashMap;
use reqwest::header::HeaderMap;


pub fn run(filename: &String) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;
        // .expect("Something went wrong reading the file");

    println!("Liste d'IP: \n{}", contents);
    println!("{:#?}", contents);

    Ok(())
}


#[tokio::main]
pub async fn virust_req(ip: &String) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("apiK_file")?;
    let api_key = content.replace("\n", "");
    // println!("{}", api_key);
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

    println!("{}", resp_json["data"]["attributes"]["last_analysis_stats"]);

    Ok(())
}
// pub struct Params {
//     pub ip: String,
//     pub filename: String,
// }
// 
// impl Params {
//     pub fn new(args: &[String]) -> Result<Params, &str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
// 
//         let ip = args[1].clone();
//         let filename = args[2].clone();
// 
//         Ok(Params { ip, filename })
//     }
// }
