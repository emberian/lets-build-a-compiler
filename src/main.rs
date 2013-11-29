use cradle::Translator;

mod cradle;

fn main() {
    let mut t = Translator::init();
    t.expression();
}
