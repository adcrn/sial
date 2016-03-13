use std::io;

struct MeanFeatures {
    wordlen: i32,	// The length of each of these
    sentlen: i32,	// characteristics will be averaged
    paralen: i32,	// and compared with other source.
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
    condition: i32,		// Did not want to create confusion with if.
    thats: i32,
    more: i32,
    musts: i32,
    mights: i32,
    thises: i32,
    very: i32,
}

fn main() {
    unimplemented!();
}