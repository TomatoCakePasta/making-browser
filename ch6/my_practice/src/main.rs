#![no_std]
#![no_main]

extern crate alloc;

use alloc::rc::Rc;
use core::cell::RefCell;
use noli::*;
use saba_core::browser::Browser;
use ui_wasabi::app::WasabiUI;

fn main() -> u64 {
    // initialize Browser constructure
    let browser = Browser::new();

    // initialize WasabiUI constructure
    let ui = Rc::new(RefCell::new(WasabiUI::new(browser)));

    // run app
    match ui.borrow_mut().start() {
        Ok(_) => {}
        Err(e) => {
            println!("borwser fails to start {:?}", e);
            return 1;
        }
    };

    0
}

entry_point!(main);