// use std::env;
use std::process;
use clap::{App, Arg};
// use ip_inspecteur::Args;
use ip_inspecteur::run;
use ip_inspecteur::virust_req;

// #[derive(Deserialize, Debug)]
// struct Ip_intel {
//     ip_sus: String
// }


fn main() {

    // let args: Vec<String> = env::args().collect();
    // let args = Args::parse();
    let app = App::new("ip_inspecteur")
        .version("0.1.3")
        .author("sanakan")
        .about("")
        .arg(
            Arg::new("ip")
                .short('i')
                .long("ip")
                .value_name("ip")
                .help("Specify an ip")
                .takes_value(true)
        )
        .arg(
            Arg::new("filename")
                .short('f')
                .long("filename")
                .value_name("filename")
                .help("Specify a filename")
                .takes_value(true)
        )
        .after_help("help display")
        .get_matches();
    
    // virust_req(&args.ip);
    
//     let params = Params::new(&args).unwrap_or_else( |err | {
//         println!("Problem parsing arguments: {}", err);
//         process::exit(1);
//     });

    // println!("IP adress: {}\n", args.ip);

    if let Some(ip) = app.value_of("ip") {
        if let Err(e) = virust_req(&ip.to_string()) {
            println!("Application error: {}", e);
            process::exit(1);
        }
    }

    if let Some(filename) = app.value_of("filename") {
        if let Err(e) = run(&filename.to_string()) {
            println!("Application error: {}", e);
            process::exit(1);
            }
    }

}



