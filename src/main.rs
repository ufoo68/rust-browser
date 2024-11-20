#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::ToString;
use noli::prelude::*;
use saba_core::browser::Browser;
use saba_core::http::HttpResponse;

static TEST_HTTP_RESPONSE : &str = r#"HTTP/1.1 200 OK
Data: Wed, 21 Oct 2015 07:28:00 GMT

<html>
    <head>
        <title>Test</title>
    </head>
    <body>
        <h1>Test</h1>
    </body>
</html>
"#;
fn main() -> u64 {
    let browser = Browser::new();
    let response = HttpResponse::new(TEST_HTTP_RESPONSE.to_string()).expect("Failed to create HttpResponse");
    let page = browser.borrow().current_page();
    let dom_string = page.borrow_mut().receive_response(response);

    for log in dom_string.lines() {
        println!("{}", log);
    }
    0
}

entry_point!(main);