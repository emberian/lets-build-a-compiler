use std::io::stdin;
use std::ascii::Ascii;

pub struct Translator {
    reader: Box<Reader>,
    look: Ascii,
    labels: uint,
}

impl Translator {
    pub fn init() -> Translator {
        let mut t = Translator {
            look: '\0'.to_ascii(),
            reader: box stdin(),
            labels: 0,
        };
        t.read(); // this is important! reads the first char of input
        t
    }

    /// Get the current lookahead character
    pub fn look(&self) -> char {
        self.look.to_char()
    }

    /// Read the next character of input
    pub fn read(&mut self) {
        self.look = self.reader.read_byte().ok()
                        .expect("expected another character").to_ascii();
    }

    /// Check if the current character is `c`, fail otherwise
    pub fn match_(&mut self, c: char) {
        if self.look == c.to_ascii() {
            self.read();
        } else {
            expected(c.to_str().as_slice());
        }
    }

    /// Get an identifier
    pub fn get_name(&mut self) -> Ascii {
        let l = self.look;
        if !l.is_alphabetic() {
            expected("Name");
        }
        self.read();
        l.to_uppercase()
    }

    /// Generate a unique label
    fn new_label(&mut self) -> String {
        let res = format!("L{}", self.labels);
        self.labels += 1;
        res
    }

    /// Post a label to output
    fn post_label(&self, label: &str) {
        println!("{}:", label);
    }

    /// Parse an "other"?
    pub fn other(&mut self) {
        emitln(self.get_name().to_str().as_slice());
    }

    /// <program> ::= <block> END
    pub fn program(&mut self) {
        self.block();
        if self.look.to_char() != 'e' {
            expected("End");
        }
        emitln("END");
    }

    /// <block> ::= [ <statement> ]*
    pub fn block(&mut self) {
        while self.look.to_char() != 'e' {
            self.other();
        }
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
