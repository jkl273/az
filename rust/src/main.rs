extern crate hyper;
extern crate encoding;
extern crate rand;
extern crate rustc_serialize;
extern crate regex;
extern crate clap;

#[macro_use]
extern crate log;
extern crate env_logger;

use clap::{Arg, App};
use hyper::client::Client;
use std::io::prelude::*;
use rand::distributions::{IndependentSample, Range};
use rustc_serialize::json::Json;
use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_31J; // shift jis
use regex::Regex;

static MAXIDX: u32 = 100;
static MAXLEN: usize = 1000;
static URL000: &'static str = "http://www.aozorahack.net/api/v0.1/";
static PUNCT: &'static str = "\u{3002}"; // 'IDEOGRAPHIC FULL STOP' (U+3002)

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
    debug!("{:?}", res);

    let mut buffer = Vec::new();
    res.read_to_end(&mut buffer).unwrap();

    WINDOWS_31J.decode(&buffer, DecoderTrap::Replace).unwrap()
}

fn next(state: u32, line: String) -> u32 {
    let re = Regex::new(r"^--*$").unwrap();
    match state {
        0 => { // head
            if line.trim() == "" {
                return 1
            } else {
                return 0
            }
        }
        1 => { // post-head
            if line.trim() == "" {
                return 1
            } else if re.is_match(line.trim()) {
                return 2
            } else {
                return 4
            }
        }
        2 => { // comment
            if re.is_match(line.trim()) {
                return 3
            } else {
                return 2
            }
        }
        3 => { // post-comment
            if line.trim() == "" {
                return 3
            } else {
                return 4
            }
        }
        4 => { // body
            return 4
        }
        _ => { // after-body
            return 5
        }
    }
}

fn summary(body: String) -> String {
    let empty: String  = "".to_string();
    
    let lines = body.lines();
    let (ret, _) =
        lines.fold((empty, 0),
                   |(acc, oldstate), line| {
                       debug!("oldstate: {}", oldstate);
                       let state = next(oldstate, line.to_string());
                       debug!("state: {}, line: {}", state, line);
                       match state {
                           0 => { // head
                               if line.len() >= MAXLEN {
                                   (format!("{}\nline too long: {}\n",
                                            acc, line.len()),
                                    state)
                               } else {
                                   (acc + line + "\n", state)
                               }
                           }
                           1 => { // post-head
                               (format!("{}=======\n", acc), state)
                           }
                           2 => { // comment
                               (acc, state)
                           }
                           3 => { // post-comment
                               (acc, state)
                           }
                           4 => { // body
                               let (num, vvv) = contline(line.to_string(), 1);
                               if num > 0 {
                                   // get more
                                   (acc + &vvv.to_string(), state)
                               } else {
                                   // no more
                                   (acc + &vvv.to_string(), state + 1)
                               }
                           }
                           _ => (acc, state) // todo: use try_fold to break
                       }
                   });
    ret
}

fn contline(line: String, num: u32) -> (u32, String) {
    let ret = num;
    let pos = line.find(PUNCT).unwrap_or(line.len());
    debug!("pos: {}, line.len(): {}", pos, line.len());
    if line.trim() == "" {
        (ret, "".to_string())
    } else if pos == line.len() { // todo
        (ret -1, line)
    } else {
        (ret -1, format!("{}{}", line[..pos].to_string(), PUNCT))
    }
}

fn randombid() -> u64 {
    let mut rng = rand::thread_rng();
    let between = Range::new(0, MAXIDX);
    let idx = between.ind_sample(&mut rng);
    debug!("{}", idx);
    getidx(idx)
}
    
fn main() {
    env_logger::init().unwrap();
    let matches = App::new("az")
        .version("1.0")
        .arg(Arg::with_name("bookid")
             .help("book id")
             .required(false)
             .index(1)).get_matches();
    let bookid: String =
        if let Some(bid000) =  matches.value_of("bookid") {
            bid000.to_string()
        } else {
            randombid().to_string()
        };
    
    println!("book id: {}", bookid);
    let body = getbook(bookid.to_string());
    let sum = summary(body);

    println!("{}", sum)
}
