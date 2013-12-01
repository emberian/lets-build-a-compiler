use std::io::stdin;
use std::ascii::Ascii;

pub struct Translator {
    priv reader: ~Reader,
    priv look: Ascii,
}

impl Translator {
    pub fn init() -> Translator {
        let mut t = Translator {
            look: '\0'.to_ascii(),
            reader: ~stdin() as ~Reader
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
        self.look = self.reader.read_byte()
                        .expect("expected another character").to_ascii();
    }

    /// Check if the current character is `c`, fail otherwise
    pub fn match_(&mut self, c: char) {
        if self.look == c.to_ascii() {
            self.read();
        } else {
            expected(c.to_str());
        }
    }

    /// Get an identifier
    pub fn get_name(&mut self) -> Ascii {
        let l = self.look;
        if !l.is_alpha() {
            expected("Name");
        }
        self.read();
        l.to_upper()
    }

    /// Get a number
    pub fn get_num(&mut self) -> int {
        let mut value = 0;
        if !self.look.is_digit() {
            expected("Integer");
        }

        while self.look.is_digit() {
            value = (10 * value) + from_str(self.look.to_str()).unwrap();
            self.read();
        }

        value
    }

    /// Parse and translate a math term
    pub fn term(&mut self) -> int {
        let mut value = self.get_num();
        loop {
            match self.look.to_char() {
                '*' => {
                    self.match_('*');
                    value *= self.factor();
                },
                '/' => {
                    self.match_('/');
                    value /= self.factor();
                },
                _ => break
            }
        }

        value
    }

    /// Parse and translate an expression
    pub fn expression(&mut self) -> int {
        let mut value;
        if is_addop(self.look.to_char()) {
            value = 0;
        } else {
            value = self.term();
        }

        while is_addop(self.look.to_char()) {
            match self.look.to_char() {
                '+' => { self.match_('+'); value += self.term() }
                '-' => { self.match_('-'); value -= self.term() }
                _ => expected("Addop")
            }
        }

        value
    }

    /// Parse and translate a math factor
    pub fn factor(&mut self) -> int {
        let val;
        if self.look.to_char() == '(' {
            self.match_('(');
            val = self.expression();
            self.match_(')');
        } else {
            val = self.get_num();
        }

        val
    }
}

pub fn is_addop(c: char) -> bool {
    c == '+' || c == '-'
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
