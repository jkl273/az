extern crate hyper;
extern crate encoding;
extern crate rand;
extern crate rustc_serialize;
extern crate regex;

use hyper::client::Client;
use std::io::prelude::*;
use rand::distributions::{IndependentSample, Range};
use rustc_serialize::json::Json;
use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_31J; // shift jis
use regex::Regex;

static MAXIDX: u32 = 100;
static URL000: &'static str = "http://www.aozorahack.net/api/v0.1/";
static PUNCT: &'static str = "\u{3002}";

fn getbook(bid: String) -> String {
    let url2 = format!("{url}books/{bid}/content", url=URL000, bid=bid);
    getaz(url2)
}

fn getidx(idx: u32) -> u64 {
    let url2 = format!("{url}books?limit=1&skip={idx}", url=URL000, idx=idx);
    let data = Json::from_str(getaz(url2).as_str()).unwrap();
    let arr = data.as_array().unwrap();
    arr[0]["book_id"].as_u64().unwrap()
}

fn getaz(url: String) -> String {
    let client = Client::new();
    let mut res = client.get(url.as_str()).send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    //println!("{:?}", res);

    let mut buffer = Vec::new();
    res.read_to_end(&mut buffer).unwrap();

    WINDOWS_31J.decode(&buffer, DecoderTrap::Replace).unwrap()
}

fn next(state: u32, line: String) -> u32 {
    let re = Regex::new(r"^--*$").unwrap();
    if state == 0 { // head
        if line.trim() == "" {
            return 1
        } else {
            return 0
        }
    } else if state == 1 { // post-head
        if line.trim() == "" {
            return 1
        } else if re.is_match(line.trim()) {
            return 2
        } else {
            return 4
        }
    } else if state == 2 { // comment
        if re.is_match(line.trim()) {
            return 3
        } else {
            return 2
        }
    } else if state == 3 { // post-comment
        if line.trim() == "" {
            return 3
        } else {
            return 4
        }
    } else { // body
        return 4
    }
}

fn summary(body: String) -> String {
    let mut state: u32 = 0;
    let mut ret: String  = "".to_string();
    
    let lines = body.lines();
    for line in lines {
        //println!("state: {}", state);
        state = next(state, line.to_string());
        //println!("state: {}, line: {}", state, line);
        if state == 0 { // head
            ret = format!("{}{}\n", ret, line);
        } else if state == 1 { // post-head
            ret = format!("{}\n=======\n", ret)
        } else if state == 2 { // comment
            //
        } else if state == 3 { //post-comment
            //
        } else if state == 4 { // body
            let (num, vvv) = contline(line.to_string(), 1);
            ret = format!("{}{}", ret, vvv);
            if num > 0 {
                continue
            } else {
                break
            }
        }
    }
    ret
}

fn contline(line: String, num: u32) -> (u32, String) {
    let ret = num;
    let tt = line.find(PUNCT).unwrap_or(line.len());
    if line.trim() == "" {
        (ret, "".to_string())
    } else if tt == line.len() { // todo
        (ret -1, line)
    } else {
        (ret -1, line) // todo
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let between = Range::new(0, MAXIDX);
    let idx = between.ind_sample(&mut rng);
    //println!("{}", idx);
    
    let bookid = getidx(idx);
    println!("book id: {}", bookid);
    let body = getbook(bookid.to_string());
    let sum = summary(body);

    println!("{}", sum)
}
