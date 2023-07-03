use babyrs::establish_connection;
use log::info;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();
    info!("Welcome to babyrs!");

    establish_connection();
}
