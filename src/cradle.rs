use std::io::stdin;
use std::ascii::Ascii;

pub struct Cradle {
    priv reader: ~Reader,
    priv lookahead: Ascii,
}

impl Cradle {
    pub fn init() -> Cradle {
        let mut c = Cradle {
            lookahead: '\0'.to_ascii(),
            reader: ~stdin() as ~Reader
        };
        c.read();
        c
    }

    /// Read the next character of input
    pub fn read(&mut self) {
        self.lookahead = self.reader.read_byte().expect("expected another character").to_ascii();
    }

    /// Check if the current character is `c`, fail otherwise
    pub fn match_(&mut self, c: Ascii) {
        if self.lookahead == c {
            self.read();
        } else {
            expected(c.to_str());
        }
    }

    /// Get an identifier
    pub fn get_name(&mut self) -> Ascii {
        let l = self.lookahead;
        if !l.is_alpha() {
            expected("Name");
        }
        self.read();
        l.to_upper()
    }

    /// Get a number
    pub fn get_num(&mut self) -> Ascii {
        let l = self.lookahead;
        if !l.is_digit() {
            expected("Integer");
        }
        self.read();
        l
    }
}

/// Report error
pub fn error(s: &str) {
    println!("Error: {}.", s);
}

/// Report error and exit
pub fn abort(s: &str) -> ! {
    error(s);
    fail!();
}

/// Report what was expected and exit
pub fn expected(s: &str) -> ! {
    println!("Error: {} expected.", s);
    fail!();
}

/// Output a string with tab
pub fn emit(s: &str) {
    print!("\t{}", s);
}

/// Output a string with tab and newlnie
pub fn emitln(s: &str) {
    println!("\t{}", s);
}
