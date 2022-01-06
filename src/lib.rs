use std::fs;

pub fn run(params: Params) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(params.filename)?;
        // .expect("Something went wrong reading the file");

    println!("Test de lecture: \n{}", contents);

    Ok(())
}

pub struct Params {
    pub ip: String,
    pub filename: String,
}

impl Params {
    pub fn new(args: &[String]) -> Result<Params, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let ip = args[1].clone();
        let filename = args[2].clone();

        Ok(Params { ip, filename })
    }
}
