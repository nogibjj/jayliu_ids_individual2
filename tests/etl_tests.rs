use jay_liu_sqlite::{extract, query, transform_load};

#[test]
fn test_extract() {
    let url = "https://github.com/fivethirtyeight/data/blob/master/births/US_births_2000-2014_SSA.csv?raw=true";
    let file_path = "data/US_birth.csv";
    let directory = "data";

    extract(url, file_path, directory);

    assert!(std::fs::metadata(file_path).is_ok());
}

#[test]
fn test_transform_load() {
    let dataset = "data/US_birth.csv";
    let result = transform_load(dataset);

    assert_eq!(result.unwrap(), "US_BirthDB.db");
}

#[test]
fn test_query() {
    // Execute a SELECT query
    let select_query = "SELECT * FROM BirthData WHERE id = 1;";
    let result = query(select_query);

    if let Err(e) = &result {
        eprintln!("Query Error: {:?}", e);
    }

    assert!(result.is_ok());
}