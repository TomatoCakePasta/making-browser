#![no_std]
#![no_main]

extern crate alloc;

use alloc::rc::Rc;
use core::cell::RefCell;
use noli::*;
use saba_core::browser::Browser;
use ui_wasabi::app::WasabiUI;
use crate::alloc::string::ToString;
use alloc::format;
use alloc::string::String;
use net_wasabi::http::HttpClient;
use saba_core::error::Error;
use saba_core::http::HttpResponse;
use saba_core::url::Url;

fn handle_url(url: String) -> Result<HttpResponse, Error> {
    let parsed_url = match Url::new(url.to_string()).parse() {
        Ok(url) => url,
        Err(e) => {
            return Err(Error::UnexpectedInput(format!(
                "input html is not supported: {:?}",
                e
            )));
        }
    };

    // send http request
    let client = HttpClient::new();
    let response = match client.get(
        parsed_url.host(),
        parsed_url.port().parse::<u16>().expect(&format!(
            "port number should be u16 but got {}",
            parsed_url.port()
        )),
        parsed_url.path(),
    ) {
        // able to get HTTP response
        Ok(res) => {
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

fn main() -> u64 {
    // initialize Browser constructure
    let browser = Browser::new();

    // initialize WasabiUI constructure
    let ui = Rc::new(RefCell::new(WasabiUI::new(browser)));

    // run app
    match ui.borrow_mut().start(handle_url) {
        Ok(_) => {}
        Err(e) => {
            println!("borwser fails to start {:?}", e);
            return 1;
        }
    };

    0
}

entry_point!(main);