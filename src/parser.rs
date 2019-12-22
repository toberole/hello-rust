

pub struct Config {
    query: String,
    fileName: String,
}

impl Config {
//    fn new(args: &[String]) -> Config {
//        let query = args[1].clone();
//        let fileName = args[2].clone();
//
//        Config { query, fileName }
//    }

    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let query = args[1].clone();
        let fileName = args[2].clone();

        Ok(Config {
            query,
            fileName,
        })
    }
}

pub fn parse_config(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    let query = args[1].clone();
    let fileName = args[2].clone();
    Config { query, fileName }
}


