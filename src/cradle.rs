use std::io::stdin;
use std::ascii::Ascii;

pub struct Translator {
    priv reader: ~Reader,
    priv lookahead: Ascii,
}

impl Translator {
    pub fn init() -> Translator {
        let mut t = Translator {
            lookahead: '\0'.to_ascii(),
            reader: ~stdin() as ~Reader
        };
        t.read();
        t
    }

    /// Read the next character of input
    pub fn read(&mut self) {
        self.lookahead = self.reader.read_byte().expect("expected another character").to_ascii();
    }

    /// Check if the current character is `c`, fail otherwise
    pub fn match_(&mut self, c: char) {
        if self.lookahead == c.to_ascii() {
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

    /// Parse and translate a math factor
    pub fn factor(&mut self) {
        emitln(format!("mov rax, {}", self.get_num().to_str()));
    }

    /// Recognize and translate a Multiply
    pub fn multiply(&mut self) {
        self.match_('*');
        self.factor();
        emitln("mul rax, [rsp]");
        emitln("pop");
    }

    /// Recognize and translate a Divide
    pub fn divide(&mut self) {
        self.match_('/');
        self.factor();
        emitln("pop rbx");
        emitln("div rax, rbx");
    }

    /// Recognize and translate an Add
    pub fn add(&mut self) {
        self.match_('+');
        self.term();
        emitln("add rax, [rsp]");
        emitln("pop");
    }

    /// Recognize and translate a Subtract
    pub fn subtract(&mut self) {
        self.match_('-');
        self.term();
        emitln("sub rax, [rsp]");
        emitln("pop");
        emitln("neg rax");
    }

    /// Parse and translate a math term
    pub fn term(&mut self) {
        self.factor();
        let ops = ['*', '/'];
        while ops.contains(&self.lookahead.to_char()) {
            emitln("push rax");
            match self.lookahead.to_char() {
                '*' => self.multiply(),
                '/' => self.divide(),
                _ => expected("Mulop")
            }
        }

    }

    /// Parse and translate an expression
    pub fn expression(&mut self) {
        self.term();
        let ops = ['+', '-'];
        while ops.contains(&self.lookahead.to_char()) {
            emitln("push rax");
            match self.lookahead.to_char() {
                '+' => self.add(),
                '-' => self.subtract(),
                _ => expected("Addop")
            }
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
