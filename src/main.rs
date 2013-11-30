use translator::Translator;

mod translator;

fn main() {
    let mut t = Translator::init();
    t.expression();
}
