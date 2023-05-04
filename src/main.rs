use clap::{crate_authors, Parser, ValueEnum};

use crate::scrapper::jibble_in_using_web_driver;

mod scrapper;
mod web_driver;

#[derive(Debug, ValueEnum, Clone)]
pub enum WebDriver {
    Firefox,
    Chrome,
    Edge,
    Safari,
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Jibble username
    #[arg(short, long)]
    email: String,
    /// Jibble password
    #[arg(short, long)]
    password: String,
    /// Browser driver to use.
    ///
    /// The browser needs to be installed because the driver version will match with the installed browser version. The web drivers default download location is ~/.cache/selenium/
    #[arg(short, long, value_enum, default_value_t = WebDriver::Chrome)]
    driver: WebDriver,
}

fn main() {
    let args = Args::parse();

    let email = args.email;
    let password = args.password;
    let (driver_path, driver_capabilities) = web_driver::prepare_driver(args.driver);
    let mut guard = web_driver::start_web_driver(driver_path);
    jibble_in_using_web_driver(4444, &email, &password, driver_capabilities);
    guard.0.kill().expect("Failed to kill the process");
    guard.0.wait().expect("Failed to wait Web Driver to end");
}
