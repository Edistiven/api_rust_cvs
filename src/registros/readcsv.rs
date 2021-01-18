use std::error::Error;

use csv;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    // Creates a new csv `Reader` from a file
    let mut reader = csv::Reader::from_path(path)?;

    // Retrieve and print header record
    let headers = reader.headers()?;
    println!("{:?}", headers);

    // `.records` return an iterator of the internal
    // record structure
    for result in reader.records() {
        let record = result?;

        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    // If an error occurs print error
    if let Err(e) = read_from_file("./info.csv") {
        eprintln!("{}", e);
    }
}