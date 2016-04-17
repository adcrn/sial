#![allow(dead_code)]

use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::error::Error;
use std::default::Default;
use std::char;

mod tokenizer;      // Tokenizes files into words, sentences, and paragraphs.
mod calculator;     // Does all the calculations

#[derive(Default, Copy, Clone)]
struct Metadata {
    word_total: f32,    // Intermediary for calculator.
    wordlen: f32,
    sentlen: f32,
    paralen: i32,
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
    
    let mut off = Metadata {..Default::default()};
    let mut pseu = Metadata {..Default::default()};

    let args: Vec<String> = env::args().collect();
    
    let mut file1 = match File::open(&args[1]) {
        Err(why) => panic!("Couldn't open {}: {}", &args[1], Error::description(&why)),
        Ok(file1) => file1,
    };
    let mut file2 = match File::open(&args[2]) {
        Err(why) => panic!("Couldn't open {}: {}", &args[2], Error::description(&why)),
        Ok(file2) => file2,
    };

    let (mut s1, mut s2) = (String::new(), String::new());

    match file1.read_to_string(&mut s1) {
        Err(why) => panic!("Couldn't read file1: {}", Error::description(&why)),
        Ok(_) => println!("File 1 read."),
    };
    
    match file2.read_to_string(&mut s2) {
        Err(why) => panic!("Couldn't read file2: {}", Error::description(&why)),
        Ok(_) => println!("File 2 read."),
    };

    let s1_token: Vec<&str> = tokenizer::word_token(&s1);
    let s2_token: Vec<&str> = tokenizer::word_token(&s2);
    off.word_total = calculator::num_words(&s1_token);
    pseu.word_total = calculator::num_words(&s2_token);
    println!("Word total #1: {}", off.word_total);
    println!("Word total #2: {}", pseu.word_total);
    off.wordlen = calculator::word_mean(&s1_token);
    pseu.wordlen = calculator::word_mean(&s2_token);
    println!("Mean word length #1: {}", off.wordlen);
    println!("Mean word length #2: {}", pseu.wordlen);

    let s1_sent: Vec<&str> = tokenizer::sent_token(&s1);
    let s2_sent: Vec<&str> = tokenizer::sent_token(&s2);
    off.sentlen = calculator::sent_mean(&off.word_total, &s1_sent);
    pseu.sentlen = calculator::sent_mean(&pseu.word_total, &s2_sent);
    println!("Mean sentence length #1: {}", off.sentlen);
    println!("Mean sentence length #2: {}", pseu.sentlen);

}