use std::path::PathBuf;
use std::process::{Child, Command};

use selenium_manager::chrome::ChromeManager;
use selenium_manager::edge::EdgeManager;
use selenium_manager::firefox::FirefoxManager;
use selenium_manager::safari::SafariManager;
use selenium_manager::SeleniumManager;
use thirtyfour::{
    Capabilities, ChromeCapabilities, EdgeCapabilities, FirefoxCapabilities, SafariCapabilities,
};

use crate::WebDriver;

pub fn get_web_driver_path<T: SeleniumManager>(browser_manager: Box<T>) -> PathBuf {
    let mut manager: Box<T> = browser_manager;
    let driver_version = manager
        .discover_driver_version()
        .expect("Browser was not found");

    manager.set_driver_version(driver_version);
    manager
        .download_driver()
        .expect("Error downloading the driver...");
    manager.get_driver_path_in_cache()
}

pub fn prepare_driver(driver: WebDriver) -> (PathBuf, Capabilities) {
    match driver {
        WebDriver::Firefox => (
            get_web_driver_path(FirefoxManager::new().unwrap()),
            Capabilities::from(FirefoxCapabilities::default()),
        ),
        WebDriver::Chrome => (
            get_web_driver_path(ChromeManager::new().unwrap()),
            Capabilities::from(ChromeCapabilities::default()),
        ),
        WebDriver::Edge => (
            get_web_driver_path(EdgeManager::new().unwrap()),
            Capabilities::from(EdgeCapabilities::default()),
        ),
        WebDriver::Safari => (
            get_web_driver_path(SafariManager::new().unwrap()),
            Capabilities::from(SafariCapabilities::default()),
        ),
    }
}

pub fn start_web_driver(path: PathBuf) -> ChildGuard {
    let child = Command::new(path)
        .arg("--port=4444")
        .spawn()
        .expect("Error initializing the driver");
    ChildGuard(child)
}

pub struct ChildGuard(pub Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        match self.0.kill() {
            Err(e) => eprintln!("Could not kill child process: {}", e),
            Ok(_) => println!("Successfully killed child process"),
        }
    }
}
