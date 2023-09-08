use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String, 
    pub file_path: String, 
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config{query, file_path})
    }
    
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    let results = get_value_by_key(&config.query, &contents);
    println!("With text: \n {:?}", results);
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    println!("result is : {:?}", results.first());

    results
}

pub fn get_value_by_key<'a>(
    keyword: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let keyword = keyword.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&keyword) {
           // results.push(line);
           let v: Vec<&str> = line.split("=").collect();
           println!("properties key is :{} " , v[0]);
           println!("properties value is :{} " , v[1]);
           results.push(v[1]);

        }
    }

    results
}
