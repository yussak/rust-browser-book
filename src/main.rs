#![no_std]
#![no_main]

extern crate alloc;

use core::cell::RefCell;

use crate::alloc::string::ToString;
use alloc::rc::Rc;
use noli::prelude::*;
use saba_core::{browser::Browser, http::HttpResponse};
use ui_wasabi::app::WasabiUI;

// MEMO:ブラウザ画面を起動するコマンド
// rust-browser-book % DISPLAY=1 ./run_on_wasabi.sh

fn main() -> u64 {
    let browser = Browser::new();
    let ui = Rc::new(RefCell::new(WasabiUI::new(browser)));

    match ui.borrow_mut().start() {
        Ok(_) => {}
        Err(e) => {
            println!("browser fails to start {:?}", e);
            return 1;
        }
    };
    0
}

entry_point!(main);
