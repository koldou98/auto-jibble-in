use thirtyfour::{By, EdgeCapabilities, WebDriver};
use thirtyfour::prelude::{ElementQueryable, ElementWaitable};

#[tokio::main]
pub async fn create_web_driver(port: u16, email: &str, password: &str) {
    let driver = start_web_driver_session(port).await;
    driver
        .goto("https://web.jibble.io/login")
        .await
        .expect("Error going to wikipedia");

    login_into_jibble(&driver, password, email).await;
    jibble_in(&driver).await;

    driver.quit().await.expect("Error the closing the driver");
}

async fn start_web_driver_session(port: u16) -> WebDriver {
    let caps = EdgeCapabilities::new();
    WebDriver::new(format!("http://localhost:{port}").as_str(), caps)
        .await
        .expect("Error connecting to the web driver")
}

async fn jibble_in(driver: &WebDriver) {
    let clock_in_button_xpath = "/html/body/div[1]/div[1]/div[1]/header/div/div/div/div/div[2]/button";
    let clock_in_save_button_xpath = "/html/body/div[1]/div[2]/div[1]/aside/div[1]/div/div[3]/button[2]";
    let clock_in_button = driver.query(By::XPath(clock_in_button_xpath)).first().await.expect("Clock in button was not found");
    if clock_in_button.class_name().await.expect("Error getting the class of the Jibble in button").unwrap().contains("success") {
        clock_in_button.click().await.expect("Jibble in failed");
        let clock_in_save_button = driver.query(By::XPath(clock_in_save_button_xpath)).first().await.expect("Jibble in save button was not found");
        clock_in_save_button.click().await.expect("Error saving Jibble in");
        clock_in_save_button.wait_until().error("Timed out waiting for button to become stale").stale().await.expect("Jibble button is still visible");
    } else {
        panic!("You are already Jibble in")
    };
}

async fn login_into_jibble(driver: &WebDriver, password: &str, email: &str) {
    let email_input_xpath = "/html/body/div[1]/div/div[1]/div/div/div[2]/div/div/div/div/div[2]/form/div[1]/div/div[1]/div/input";
    let password_input_xpath = "/html/body/div[1]/div/div[1]/div/div/div[2]/div/div/div/div/div[2]/form/div[2]/div/div/div/div[1]/input";
    let login_button_xpath = "/html/body/div[1]/div/div[1]/div/div/div[2]/div/div/div/div/div[2]/button";

    fill_input_by_xpath(driver, email_input_xpath, email, "user or phone number").await;
    fill_input_by_xpath(driver, password_input_xpath, password, "password").await;
    let login_button = driver.query(By::XPath(login_button_xpath)).first().await.expect("Logging button was not found");
    login_button.click().await.expect("Error when logging...");
}

async fn fill_input_by_xpath(driver: &WebDriver, xpath: &str, value: &str, input_description: &str) {
    let email_input = driver
        .query(By::XPath(xpath))
        .first()
        .await
        .unwrap_or_else(|_| panic!("Input for {input_description} can not be found"));
    email_input
        .send_keys(value)
        .await
        .unwrap_or_else(|_| panic!("Error filling {input_description}"));
}
