extern crate hyper;
extern crate encoding;

use hyper::client::Client;
use std::io::prelude::*;

use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_31J; // shift jis

fn getbook(bid: &str) -> String {
    let client = Client::new();
    let url = "http://www.aozorahack.net/api/v0.1/";
    let url2 = format!("{url}books/{bid}/content", url=url, bid=bid);
    let mut res = client.get(url2.as_str()).send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    println!("{:?}", res);

    let mut buffer = Vec::new();
    res.read_to_end(&mut buffer).unwrap();

    WINDOWS_31J.decode(&buffer, DecoderTrap::Replace).unwrap()
}

fn summary(body: String) -> String {
    let mut ret: String = "".to_string();
    let lines = body.lines();
    for line in lines {
        ret = format!("{}{}\n", ret, line);
    }
    ret
}

fn main() {
    let body = getbook("1234");
    let sum = summary(body);

    println!("{}", sum)
}
