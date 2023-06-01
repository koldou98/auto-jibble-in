
use std::thread::sleep;
use std::time::Duration;

use thirtyfour::prelude::{ElementQueryable, ElementWaitable};
use thirtyfour::{By, Capabilities, WebDriver};

#[tokio::main]
pub async fn jibble_in_using_web_driver(
    port: u16,
    email: &str,
    password: &str,
    capabilities: Capabilities,
) {
    let driver = start_web_driver_session(port, capabilities).await;

    login_into_jibble(&driver, password, email).await;
    jibble_in(&driver).await;
    driver.quit().await.expect("Error the closing the driver");
}

async fn start_web_driver_session(port: u16, capabilities: Capabilities) -> WebDriver {
    let max_connection_attempts = 3;
    for attempt in 0..max_connection_attempts {
        let result = WebDriver::new(
            format!("http://localhost:{port}").as_str(),
            capabilities.clone(),
        )
        .await;
        if result.is_err() {
            if attempt.eq(&max_connection_attempts) {
                panic!("Could not connect to the web server");
            }
            eprintln!("Failed to connect to the web driver waiting 1 second before the next try");
            sleep(Duration::from_secs(1));
        } else {
            return result.unwrap();
        }
    }
    panic!("Error starting web driver session");
}

async fn jibble_in(driver: &WebDriver) {
    let optional_recommendation_close_button_xpath = "/html/body/div[1]/div[3]/div/div/button";
    let clock_in_button_xpath =
        "/html/body/div[1]/div[1]/div[1]/header/div/div/div/div/div[2]/button";
    let clock_in_save_button_xpath =
        "/html/body/div[1]/div[2]/div[1]/aside/div[1]/div/div[3]/button[2]";
    if let Ok(optional_recommendation_close_button) = driver
        .query(By::XPath(optional_recommendation_close_button_xpath))
        .wait(Duration::from_secs(2), Duration::from_millis(100))
        .first()
        .await
    {
        if optional_recommendation_close_button.click().await.is_err() {
            close_driver_on_error(driver, "Could not close recommendation message").await;
        };
    }
    let clock_in_button = match driver.query(By::XPath(clock_in_button_xpath)).first().await {
        Ok(web_element) => web_element,
        Err(_) => {
            close_driver_on_error(driver, "Clock in button was not found").await;
            return;
        }
    };
    if clock_in_button
        .class_name()
        .await
        .expect("Error getting the class of the Jibble in button")
        .unwrap()
        .contains("success")
    {
        clock_in_button.click().await.expect("Jibble in failed");
        let clock_in_save_button = match driver
            .query(By::XPath(clock_in_save_button_xpath))
            .first()
            .await
        {
            Ok(element) => element,
            Err(_) => {
                close_driver_on_error(driver, "Error saving Jibble in").await;
                return;
            }
        };
        if (clock_in_save_button.click().await).is_err() {
            close_driver_on_error(driver, "Error saving Jibble in").await;
        }
        if (clock_in_save_button
            .wait_until()
            .error("Timed out waiting for button to become stale")
            .stale()
            .await)
            .is_err()
        {
            close_driver_on_error(driver, "Jibble button is still visible").await;
        };
    } else {
        eprintln!("You are already Jibble in")
    };
}

async fn close_driver_on_error(driver: &WebDriver, error_message: &str) {
    eprintln!("{}", error_message);
    driver
        .clone()
        .quit()
        .await
        .expect(" Error closing the driver");
}

async fn login_into_jibble(driver: &WebDriver, password: &str, email: &str) {
    let email_input_xpath = "/html/body/div[1]/div/div[1]/div/div/div[2]/div/div/div/div/div[2]/form/div[1]/div/div[1]/div/input";
    let password_input_xpath = "/html/body/div[1]/div/div[1]/div/div/div[2]/div/div/div/div/div[2]/form/div[2]/div/div/div/div[1]/input";
    let login_button_xpath =
        "/html/body/div[1]/div/div[1]/div/div/div[2]/div/div/div/div/div[2]/button";

    if driver.goto("https://web.jibble.io/login").await.is_err() {
        close_driver_on_error(driver, "Error going to Jibble").await;
    }

    fill_input_by_xpath(driver, email_input_xpath, email, "user or phone number").await;
    fill_input_by_xpath(driver, password_input_xpath, password, "password").await;
    let login_button = match driver.query(By::XPath(login_button_xpath)).first().await {
        Ok(web_element) => web_element,
        Err(_) => {
            close_driver_on_error(driver, "Logging button was not found").await;
            return;
        }
    };
    if login_button.click().await.is_err() {
        close_driver_on_error(driver, "Error when logging...").await;
    }
}

async fn fill_input_by_xpath(
    driver: &WebDriver,
    xpath: &str,
    value: &str,
    input_description: &str,
) {
    let input = match driver.query(By::XPath(xpath)).first().await {
        Ok(web_element) => web_element,
        Err(_) => {
            close_driver_on_error(
                driver,
                format!("Input for {} can not be found", input_description).as_str(),
            )
            .await;
            return;
        }
    };
    if input.send_keys(value).await.is_err() {
        close_driver_on_error(
            driver,
            format!("Error filling {}", input_description).as_str(),
        )
        .await;
    };
}
