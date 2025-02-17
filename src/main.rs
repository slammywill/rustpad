mod types;

use std::error::Error;
use std::thread;
use std::time::Duration;

use types::LaunchpadDevice;


fn main() -> Result<(), Box<dyn Error>> {
    let _launchpad = LaunchpadDevice::new()?;

    loop {
        thread::sleep(Duration::from_millis(10));
    }
}
