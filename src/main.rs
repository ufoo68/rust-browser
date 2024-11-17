#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;

fn main() -> u64 {
    let client = HttpClient::new();
    match client.get("host.test".to_string(), 8000, "/test.html".to_string()) {
        Ok(response) => {
            print!("response: {:#?}", response);
        },
        Err(e) => {
            println!("Error:\n{:#?}", e);
        },
    }
    0
}

entry_point!(main);