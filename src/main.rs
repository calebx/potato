extern crate log;
extern crate simplelog;

mod boot;

use potato::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    boot::logger::init();

    loop {
        sleep(Duration::from_secs(3));
        Potato::run();
    }
}
