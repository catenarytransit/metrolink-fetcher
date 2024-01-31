use headless_chrome::protocol::cdp::Page;
use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use headless_chrome::{protocol::cdp::Page::CaptureScreenshot, Browser};
use std::{thread, time};

static TARGET: &str = "https://metrolinktrains.com/rtt/StationScheduleList.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = headless_chrome::LaunchOptionsBuilder::default()
        .window_size(Some((720, 1080)))
        .sandbox(false)
        .headless(false)
        .build()
        .unwrap();

    let browser = Browser::new(options)?;

    let tab = browser.new_tab()?;

    /// Navigate to page
    tab.navigate_to(TARGET)?;

    loop {
        println!("Starting refresh...");

        tab.reload(true, None::<&str>)?;

        println!("Waiting until navigation");

        tab.wait_until_navigated()?;

        let data = tab.get_content().unwrap();

        //todo!("upload data to Catenary distributed system here");

        let one_sec = time::Duration::from_secs(1);

        thread::sleep(one_sec);
    }

    //println!("{}", tab.get_content().unwrap());

    Ok(())
}
