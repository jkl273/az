extern crate hyper;
extern crate encoding;
extern crate rand;
extern crate rustc_serialize;

use hyper::client::Client;
use std::io::prelude::*;
use rand::distributions::{IndependentSample, Range};
use rustc_serialize::json::Json;

use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_31J; // shift jis

fn getbook(bid: String) -> String {
    let url = "http://www.aozorahack.net/api/v0.1/";
    let url2 = format!("{url}books/{bid}/content", url=url, bid=bid);
    getaz(url2)
}

fn getidx(idx: u32) -> u64 {
    let url = "http://www.aozorahack.net/api/v0.1/";
    let url2 = format!("{url}books?limit=1&skip={idx}", url=url, idx=idx);
    let data = Json::from_str(getaz(url2).as_str()).unwrap();
    let arr = data.as_array().unwrap();
    arr[0]["book_id"].as_u64().unwrap()
}

fn getaz(url: String) -> String {
    let client = Client::new();
    let mut res = client.get(url.as_str()).send().unwrap();
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
    let maxidx = 3;
    let mut rng = rand::thread_rng();
    let between = Range::new(0, maxidx);
    let idx = between.ind_sample(&mut rng);
    println!("{}", idx);
    
    let bookid = getidx(idx);
    let body = getbook(bookid.to_string());
    let sum = summary(body);

    println!("{}", sum)
}
