pub mod terminal;

use log::info;

use crate::terminal::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Welcome to babyrs!");

    // Establish connection to database
    // let connection: &mut SqliteConnection = &mut establish_connection();
    let app = App::new();
    terminal::start_ui(app)?;

    // let app = run(Duration::from_millis(200), true);

    // Process CSV file
    // process_csv(connection, "sample/example.csv").expect("Error processing CSV");
    // let events = read_events(connection);

    // for event in events {
    //     println!("{:?}", event);
    // }

    Ok(())
}
