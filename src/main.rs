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
