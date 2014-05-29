#![allow(dead_code)] // most of the code is going to be dead in any given version

use translator_control::Translator;

mod translator;
mod translator_multichar;
mod translator_interp;
mod translator_control;

fn main() {
    let mut t = Translator::init();

    t.program();
}
