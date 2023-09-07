use std::env;
use std::process;
use rust_properties::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

   let config = Config::build(&args).unwrap_or_else(|err|{
    println!("Problem parsing arguments: {err}");
    process::exit(1);
   });
   
    println!("searching for : {}", config.query);
    println!("searching for : {}", config.file_path);
    if let Err(e) = rust_properties::run(config){
        println!("Application error: {e}");
        process::exit(1);
    }
   // dbg!(args);
   // println!("Hello, world!");
}

// fn parse_config(args: &[String]) -> Result<&str, &'static str> {

//     if args.len() < 2 {
//        // panic!("not enough arguments, you may miss the file path");
    
//        return Err("not enough arguments, you may miss the file path");
//     }
//     let file_path = &args[1];
//     Ok(&file_path)
// }


