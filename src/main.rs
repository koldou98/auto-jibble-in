use std::path::PathBuf;
use std::process::{Child, Command};

use clap::Parser;
use log::{info, LevelFilter};
use selenium_manager::edge::EdgeManager;
use selenium_manager::SeleniumManager;

use crate::scrapper::create_web_driver;

mod scrapper;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Jibble username
    #[arg(short, long)]
    email: String,
    /// Jibble password
    #[arg(short, long)]
    password: String,
}

fn main() {
    let args = Args::parse();
    env_logger::builder().filter_level(LevelFilter::Debug).init();
    let email = args.email;
    let password = args.password;
    // TODO: Support multiple web drivers -> currently only Edge supported
    let mut manager = EdgeManager::new().expect("error");
    let driver_version = manager.discover_driver_version().unwrap();
    manager.set_driver_version(driver_version);
    manager.download_driver().expect("Error downloading the driver...");
    let path = manager.get_driver_path_in_cache();
    let mut child = start_web_driver(path);
    create_web_driver(4444, &email, &password);
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
