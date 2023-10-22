use xueqing_wu_individual2::{extract, query, transform_load};

#[test]
fn test_extract() {
    let url =
        "https://github.com/fivethirtyeight/data/blob/master/births/US_births_2000-2014_SSA.csv?raw=true";
    let file_path = "data/births.csv";
    let directory = "data";

    extract(url, file_path, directory);

    assert!(std::fs::metadata(file_path).is_ok());
}

#[test]
fn test_transform_load() {
    let dataset = "data/births.csv";
    let result = transform_load(dataset);

    assert_eq!(result.unwrap(), "Birth.db");
}

#[test]
fn test_query() {
    // Execute a SELECT query
    let select_query = "SELECT * FROM Birth WHERE id = 3;";
    let result = query(select_query);

    assert!(result.is_ok());
}