#![allow(dead_code)]

use std::io;
use std::env;
use std::fs::File;
use std::error::Error;
use std::path::Path;

struct MeanFeatures {
    wordlen: i32,	// The length of each of these characteristics will be
    sentlen: i32,	// averaged and compared with other source.
    //paralen: i32,	// Removing this for now.
}

struct FreqFeatures {
    comma: i32,
    semicolon: i32,
    quote: i32,
    bangs: i32,
    dashes: i32,
    ands: i32,
    buts: i32,
    however: i32,
    condition: i32,		// Prevent confusion with if.
    thats: i32,
    more: i32,
    musts: i32,
    mights: i32,
    thises: i32,
    very: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file1 = match File::open(&args[1]) {
        Err(why) => panic!("Couldn't open {}: {}", &args[1], Error::description(&why)),
        Ok(file1) => file1,
    };
    let file2 = match File::open(&args[2]) {
        Err(why) => panic!("Couldn't open {}: {}", &args[2], Error::description(&why)),
        Ok(file2) => file2,
    };
}