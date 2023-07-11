use babyrs::{establish_connection, process_csv, read_events};
use diesel::SqliteConnection;
use log::info;
use simple_logger::SimpleLogger;
use std::io::Error;

fn main() -> Result<(), Error> {
    SimpleLogger::new().init().unwrap();
    info!("Welcome to babyrs!");

    let connection: &mut SqliteConnection = &mut establish_connection();

    process_csv(connection, "sample/example.csv").expect("Error processing CSV");
    let events = read_events(connection);

    for event in events {
        println!("{:?}", event);
    }

    Ok(())
}
