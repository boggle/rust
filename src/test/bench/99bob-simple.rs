

/* -*- mode::rust;indent-tabs-mode::nil -*-
 * Implementation of 99 Bottles of Beer
 * http://99-bottles-of-beer.net/
 */
use std;
import int;
import str;

fn b1() -> str { ret "# of beer on the wall, # of beer."; }

fn b2() -> str {
    ret "Take one down and pass it around, # of beer on the wall.";
}

fn b7() -> str {
    ret "No more bottles of beer on the wall, no more bottles of beer.";
}

fn b8() -> str {
    ret "Go to the store and buy some more, # of beer on the wall.";
}

fn sub(t: str, n: int) -> str unsafe {
    let mut b: str = "";
    let mut i: uint = 0u;
    let mut ns: str;
    alt n {
      0 { ns = "no more bottles"; }
      1 { ns = "1 bottle"; }
      _ { ns = int::to_str(n, 10u) + " bottles"; }
    }
    while i < str::len(t) {
        if t[i] == '#' as u8 { b += ns; }
        else { str::unsafe::push_byte(b, t[i]); }
        i += 1u;
    }
    ret b;
}


/* Straightforward counter */
fn main() {
    let mut n: int = 99;
    while n > 0 {
        log(debug, sub(b1(), n));
        log(debug, sub(b2(), n - 1));
        #debug(""); n -= 1;
    }
    log(debug, b7());
    log(debug, sub(b8(), 99));
}
