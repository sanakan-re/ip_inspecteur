use std::fs;
use clap::Parser;

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(args.filename)?;
        // .expect("Something went wrong reading the file");

    println!("Test de lecture: \n{}", contents);

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    ///  ip
    #[clap(short, long)]
    pub ip: String,

    /// filename
    #[clap(short, long)]
    pub filename: String,
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
