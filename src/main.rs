use diesel::prelude::*;
use log::info;

use babyrs::terminal::{self, app::App};
use babyrs::{establish_connection, process_csv};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Welcome to babyrs!");

    let app = App::new();
    terminal::start_ui(app)?;

    // Process CSV file
    info!("Loading events from CSV into the database...");

    // Establish connection to database
    let connection: &mut SqliteConnection = &mut establish_connection();
    process_csv(connection, "sample/example.csv").expect("Error processing CSV");

    Ok(())
}
