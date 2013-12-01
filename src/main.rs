use translator_multichar::Translator;

mod translator;
mod translator_multichar;

fn main() {
    let mut t = Translator::init();
    t.expression();
    if t.look() != '\n' {
        translator::expected("Newline");
    }
}
