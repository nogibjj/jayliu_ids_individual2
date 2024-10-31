use reqwest::blocking::Client;
use rusqlite::{params, Connection, Result};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

const LOG_FILE: &str = "query_log.md";

fn log_query(query: &str, log_file: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(log_file) {
        if let Err(err) = writeln!(file, "```sql\n{}\n```\n", query) {
            eprintln!("Error writing to log file: {:?}", err);
        }
    } else {
        eprintln!("Error opening log file for writing.");
    }
}

pub fn extract(url: &str, file_path: &str, directory: &str) {
    if !fs::metadata(directory).is_ok() {
        fs::create_dir_all(directory).expect("Failed to create directory");
    }

    let client = Client::new();
    let mut response = client.get(url).send().expect("Failed to send request");
    let mut file = fs::File::create(file_path).expect("Failed to create file");

    std::io::copy(&mut response, &mut file).expect("Failed to copy content");

    println!("Extraction successful!");
}

pub fn transform_load(dataset: &str) -> Result<String> {
    let conn = Connection::open("US_BirthDB.db")?;

    // Drop existing table if it exists
    conn.execute("DROP TABLE IF EXISTS BirthData", [])?;

    // Create new table with columns matching US_birth.csv
    conn.execute(
        "CREATE TABLE BirthData (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            year INTEGER,
            month INTEGER,
            date_of_month INTEGER,
            day_of_week INTEGER,
            births INTEGER
        )",
        [],
    )?;

    let mut rdr = csv::Reader::from_path(dataset).expect("Failed to read dataset");

    // Prepare the insertion statement
    let mut stmt = conn.prepare(
        "INSERT INTO BirthData (
            year, 
            month, 
            date_of_month, 
            day_of_week, 
            births
        ) 
        VALUES (?, ?, ?, ?, ?)",
    )?;

    // Insert each record from the CSV into the database
    for result in rdr.records() {
        match result {
            Ok(record) => {
                stmt.execute(&[
                    &record[0],  // year
                    &record[1],  // month
                    &record[2],  // date_of_month
                    &record[3],  // day_of_week
                    &record[4],  // births
                ])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV record: {:?}", err);
            }
        }
    }

    Ok("US_BirthDB.db".to_string())
}

pub fn query(query: &str) -> Result<()> {
    let conn = Connection::open("US_BirthDB.db")?;
    // Read operation
    if query.trim().to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(query)?;
        let results = stmt.query_map(params![], |row| {
            Ok((
                row.get::<usize, i32>(0)?,   // id
                row.get::<usize, i32>(1)?,   // year
                row.get::<usize, i32>(2)?,   // month
                row.get::<usize, i32>(3)?,   // date_of_month
                row.get::<usize, i32>(4)?,   // day_of_week
                row.get::<usize, i32>(5)?,   // births
            ))
        })?;

        for result in results {
            match result {
                Ok((id, year, month, date_of_month, day_of_week, births)) => {
                    println!(
                        "Result: id={}, year={}, month={}, date_of_month={}, day_of_week={}, births={}",
                        id, year, month, date_of_month, day_of_week, births
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        // other CUD operations
        let _num_affected_rows = conn.execute_batch(query)?;
    }
    log_query(query, LOG_FILE);
    Ok(())
}
