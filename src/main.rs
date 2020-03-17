extern crate lazy_static;
extern crate log;
extern crate serde;
extern crate simplelog;

mod boot;

use boot::config::CONFIG;
use potato::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    boot::config::init();
    boot::logger::init();
    println!("config: {:?}", *CONFIG);

    loop {
        sleep(Duration::from_secs(3));
        Potato::run();
    }
}
