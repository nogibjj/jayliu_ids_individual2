use jay_liu_sqlite::{extract, query, transform_load};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [action]", args[0]);
        return;
    }

    let action = &args[1];
    match action.as_str() {
        "extract" => {
            extract(
                "https://github.com/fivethirtyeight/data/blob/master/births/US_births_2000-2014_SSA.csv?raw=true",
                "data/US_birth.csv",
                "data",
            );
        }
        "transform_load" => match transform_load("data/US_birth.csv") {
            Ok(_) => println!("Data loaded successfully!"),
            Err(err) => eprintln!("Error: {:?}", err),
        },
        "query" => {
            if let Some(q) = args.get(2) {
                if let Err(err) = query(q) {
                    eprintln!("Error: {:?}", err);
                } else {
                    println!("Query executed successfully!");
                }
            } else {
                println!("Usage: {} query [SQL query]", args[0]);
            }
        }
        _ => {
            println!("Invalid action. Use 'extract', 'transform_load', or 'query'.");
        }
    }
}
