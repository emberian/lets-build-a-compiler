use translator_interp::Translator;

mod translator;
mod translator_multichar;
mod translator_interp;

fn main() {
    let mut t = Translator::init();
    println!("{}", t.expression());
    if t.look() != '\n' {
        translator::expected("Newline");
    }
}
