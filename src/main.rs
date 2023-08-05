pub mod app;
pub mod state;
pub mod terminal;
pub mod ui;

use log::info;
use simple_logger::SimpleLogger;
use std::{cell::RefCell, rc::Rc};

use crate::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init().unwrap();
    info!("Welcome to babyrs!");

    // Establish connection to database
    // let connection: &mut SqliteConnection = &mut establish_connection();
    let app = Rc::new(RefCell::new(App::new()));
    terminal::run(app)?;

    // let app = run(Duration::from_millis(200), true);

    // Process CSV file
    // process_csv(connection, "sample/example.csv").expect("Error processing CSV");
    // let events = read_events(connection);

    // for event in events {
    //     println!("{:?}", event);
    // }

    Ok(())
}
