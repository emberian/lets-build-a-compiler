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
        t.read();
        t.skip_white();
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

    /// Skip any whitespace
    pub fn skip_white(&mut self) {
        while self.look.is_blank() {
            self.read();
        }
    }

    /// Check if the current character is `c`, fail otherwise
    pub fn match_(&mut self, c: char) {
        if self.look == c.to_ascii() {
            self.read();
            self.skip_white();
        } else {
            expected(c.to_str());
        }
    }

    /// Get an identifier
    pub fn get_name(&mut self) -> ~str {
        let mut token = ~"";
        if !self.look.is_alpha() {
            expected("Name");
        }

        while self.look.is_alnum() {
            token.push_char(self.look.to_upper().to_char());
            self.read();
        }
        self.skip_white();

        token
    }

    /// Get a number
    pub fn get_num(&mut self) -> Ascii {
        let l = self.look;
        if !l.is_digit() {
            expected("Integer");
        }
        self.read();
        self.skip_white();
        l
    }

    pub fn ident(&mut self) {
        let name = self.get_name();
        if self.look == '('.to_ascii() {
            self.match_('(');
            self.match_(')');
            emitln(format!("call {}", name.to_str()));
        } else {
            emitln(format!("mov rax, {}(rip) ; XXX is this correct?",
                            name.to_str()));
        }
    }

    /// Parse and translate a math factor
    pub fn factor(&mut self) {
        if self.look == '('.to_ascii() {
            self.match_('(');
            self.expression();
            self.match_(')');
        } else if self.look.is_alpha() {
            self.ident();
        } else {
            emitln(format!("mov rax, {}", self.get_num().to_str()));
        }
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
        while ops.contains(&self.look.to_char()) {
            emitln("push rax");
            match self.look.to_char() {
                '*' => self.multiply(),
                '/' => self.divide(),
                _ => expected("Mulop")
            }
        }

    }

    /// Parse and translate an expression
    ///
    /// Result of expression is stored in `rax`
    pub fn expression(&mut self) {
        if is_addop(self.look.to_char()) {
            emitln("xor rax, rax");
        } else {
            self.term();
        }

        while is_addop(self.look.to_char()) {
            emitln("push rax");
            match self.look.to_char() {
                '+' => self.add(),
                '-' => self.subtract(),
                _ => expected("Addop")
            }
        }
    }

    /// Parse and translate an Assignment Statement
    pub fn assignment(&mut self) {
        let name = self.get_name();
        self.match_('=');
        self.expression();
        emitln("push rbx");
        emitln(format!("lea rbx, {}(rip) ; XXX is this correct?", name));
        emitln("mov [rbx], rax");
        emitln("pop rbx");
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
