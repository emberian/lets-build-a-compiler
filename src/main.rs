use translator::Translator;

mod translator;

fn main() {
    let mut t = Translator::init();
    t.expression();
    if t.look() != '\n' {
        translator::expected("Newline");
    }
}
