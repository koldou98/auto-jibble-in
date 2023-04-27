use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::process::{Child, Command};

use clap::{Parser, ValueEnum};
use log::{info, LevelFilter};
use selenium_manager::chrome::ChromeManager;
use selenium_manager::SeleniumManager;

use crate::scrapper::jibble_in_using_web_driver;

mod scrapper;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Jibble username
    #[arg(short, long)]
    email: String,
    /// Jibble password
    #[arg(short, long)]
    password: String,
    /// Web Driver to use
    #[arg(short, long, value_enum, default_value_t = WebDriver::Docker)]
    driver: WebDriver,
}

#[derive(Debug, ValueEnum, Clone)]
enum WebDriver {
    Docker,
    Podman,
    Firefox,
    Chrome,
    Edge,
    Safari,
}

impl Display for WebDriver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WebDriver::Docker => write!(f, "Docker"),
            WebDriver::Podman => write!(f, "Podman"),
            WebDriver::Firefox => write!(f, "Firefox"),
            WebDriver::Chrome => write!(f, "Chrome"),
            WebDriver::Edge => write!(f, "Edge"),
            WebDriver::Safari => write!(f, "Safari")
        }
    }
}

fn main() {
    let args = Args::parse();
    env_logger::builder().filter_level(LevelFilter::Debug).init();
    let email = args.email;
    let password = args.password;
    // TODO: Support multiple web drivers -> currently only Edge supported
    let mut manager: Box<dyn SeleniumManager> = ChromeManager::new().expect("error");
    let driver_version = manager.discover_driver_version().unwrap();
    manager.set_driver_version(driver_version);
    manager.download_driver().expect("Error downloading the driver...");
    let path = manager.get_driver_path_in_cache();
    let mut child = start_web_driver(path);
    jibble_in_using_web_driver(4444, &email, &password);
    info!("Killing Web Driver");
    // TODO: Check that the child process dies on all the cases
    child.kill().expect("Failed to kill the process");
    let exit_code = child.wait().expect("Failed to wait on child");
    info!("{exit_code}")
}

fn start_web_driver(path: PathBuf) -> Child {
    Command::new(path)
        .arg("--port=4444")
        .spawn()
        .expect("Error initializing the driver")
}
