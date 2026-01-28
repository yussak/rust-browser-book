#![no_std]
#![no_main]

extern crate alloc;

use core::cell::RefCell;

use crate::alloc::string::ToString;
use alloc::{format, rc::Rc, string::String};
use net_wasabi::http::HttpClient;
use noli::*;
use saba_core::{browser::Browser, error::Error, http::HttpResponse, url::Url};
use ui_wasabi::app::WasabiUI;

// MEMO:ブラウザ画面を起動するコマンド
// rust-browser-book % DISPLAY=1 ./run_on_wasabi.sh

fn main() -> u64 {
    let browser = Browser::new();
    let ui = Rc::new(RefCell::new(WasabiUI::new(browser)));

    match ui.borrow_mut().start(handle_url) {
        Ok(_) => {}
        Err(e) => {
            println!("browser fails to start {:?}", e);
            return 1;
        }
    };
    0
}

entry_point!(main);

fn handle_url(url: String) -> Result<HttpResponse, Error> {
    // MEMO: 本ではurl.to_string()しているけどすでに所有権ありのString型だが必要なんだろうか…一旦そのまま書いている
    let parsed_url = match Url::new(url.to_string()).parse() {
        Ok(url) => url,
        Err(e) => {
            return Err(Error::UnexpectedInput(format!(
                "input html is not supported: {:?}",
                e
            )))
        }
    };

    // HTTPリクエストを送信する
    let client = HttpClient::new();
    let response = match client.get(
        parsed_url.host(),
        parsed_url.port().parse::<u16>().expect(&format!(
            "port number should be u16 but got {}",
            parsed_url.port()
        )),
        parsed_url.path(),
    ) {
        Ok(res) => {
            // HTTPレスポンスのステータスコードが302の時転送する（リダイレクト）
            if res.status_code() == 302 {
                let location = match res.header_value("Location") {
                    Ok(value) => value,
                    Err(_) => return Ok(res),
                };
                let redirect_parsed_url = Url::new(location);

                let redirect_res = match client.get(
                    redirect_parsed_url.host(),
                    redirect_parsed_url.port().parse::<u16>().expect(&format!(
                        "port number should be u16 but got {}",
                        parsed_url.port()
                    )),
                    redirect_parsed_url.path(),
                ) {
                    Ok(res) => res,
                    Err(e) => return Err(Error::Network(format!("{:?}", e))),
                };

                redirect_res
            } else {
                res
            }
        }
        Err(e) => {
            return Err(Error::Network(format!(
                "failed to get http response: {:?}",
                e
            )))
        }
    };

    Ok(response)
}
