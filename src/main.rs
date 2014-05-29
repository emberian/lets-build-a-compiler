#![allow(dead_code)] // most of the code is going to be dead in any given version

use translator_interp::Translator;

mod translator;
mod translator_multichar;
mod translator_interp;

fn main() {
    let mut t = Translator::init();

    while t.look() != '.' {
        match t.look() {
            '?' => t.input(),
            '!' => t.output(),
            _   => t.assignment()
        }
        t.newline();
    }
}
