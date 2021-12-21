extern crate serde_json;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use reqwest::header::HeaderMap;

// #[derive(Deserialize, Debug)]
// struct Ip_intel {
//     ip_sus: String
// }


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

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
    //let v: JsonValue = serde_json::from_value(resp_json)?;
    // let resp_jj = serde_json::from_str(resp_json);
    // let data : JsonValue = resp_jj.unwrap();
    //
    // let asn = String::from("country");
    //println!("{:#?}", resp_json.get(&asn));
    // println!("{:#?}", resp_json["data"]["attributes"]["last_analysis_stats"]);
    println!("{}", resp_json["data"]["attributes"]["last_analysis_stats"]);

    Ok(())
}


