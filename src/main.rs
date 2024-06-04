use std::collections::HashMap;

struct Lexer {
    input: String,
}

#[derive(Debug)]
enum Boolean {
    Null,
    True,
    False,
}

#[derive(Debug)]
enum Number {
    Float(f32),
    Integer(i32),
}

#[derive(Debug, PartialEq, Eq)]
enum TokenKind {
    Lbrace = 0,
    Rbrace,
    Lbracket,
    Rbracket,
    Colon,
    Boolen,
    Number,
    String,
    Comma
}

#[derive(Debug)]
enum Value {
    Boolen(Boolean),
    Number(Number),
    String(String),
    None
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    value: Value
}

impl Token {
    fn new(kind:TokenKind, value:Value) -> Self {
        Self {
            kind ,
            value
        }
    }
}

impl Lexer {
    fn new(input: String) -> Self {
        Self {
            input: input.chars().filter(|c| !c.is_ascii_whitespace()).collect(),
        }
    }

    fn number(&self, slice: &str) -> String {
        let mut end = 0;
        let mut it = slice.chars();
        while let Some(c) = it.next() {
            if !(c.is_ascii_digit() || c == '.' || c == 'e') {
                break;
            }
            end += 1;
        }
        (&slice[..end]).into()
    }

    fn string(&self, slice: &str) -> String {
        let mut end = 0;
        let mut it = slice.chars();
        while let Some(c) = it.next() {
            if c == '"' {
                break;
            }
            end += 1;
        }
        (&slice[..end]).into()
    }

    fn boolen(&self, slice: &str) -> String {
        let mut end = 0;
        let mut it = slice.chars();
        while let Some(c) =  it.next() {
            if !c.is_ascii_alphabetic() {
                break;
            }
            end += 1;
        }
        (&slice[..end]).into()
    }

    fn lex(&self) -> Vec<Token> {
        let mut index = 0;
        let mut tokens = vec![];
        while let Some(c) = self.input.chars().nth(index) {
            index += match c {
                '{' => {
                    tokens.push(Token::new(TokenKind::Lbrace, Value::None));
                    1
                }
                '}' =>  {
                    tokens.push(Token::new(TokenKind::Rbrace, Value::None));
                    1
                }
                ']' => {
                    tokens.push(Token::new(TokenKind::Rbracket, Value::None));
                    1
                }
                '[' => {
                    tokens.push(Token::new(TokenKind::Lbracket, Value::None));
                    1
                }
                ':' => {
                    tokens.push(Token::new(TokenKind::Colon, Value::None));
                    1
                }
                ',' => {
                    tokens.push(Token::new(TokenKind::Comma, Value::None));
                    1
                }
                c => {
                    if c == '"' {
                        index += 1;
                        let string = self.string(&self.input[index..]);
                        index += 1;
                        let adv = string.len();
                        tokens.push(Token::new(TokenKind::String, Value::String(string)));
                        adv
                    } else if c.is_alphabetic() {
                        let string = self.boolen(&self.input[index..]);
                        let adv = string.len();
                        tokens.push(Token::new(TokenKind::Boolen, Value::Boolen(match string.as_str() {
                            "true" => Boolean::True,
                            "false" => Boolean::False,
                            "null" => Boolean::Null,
                            s => panic!("unexpected value {}", s)
                        })));
                        adv
                    } else if c.is_ascii_digit() {
                        let string = self.number(&self.input[index..]);
                        let adv = string.len();
                        tokens.push(Token::new(TokenKind::Number, Value::Number(match string.contains(".") {
                            true => Number::Float(string.parse::<f32>().unwrap()),
                            false => Number::Integer(string.parse::<i32>().unwrap())
                        })));
                        adv
                    } else {
                        panic!("invalid char")
                    }
                }
            }
        }

        tokens
    }
}


struct Parser {
    current: usize,
    tokens: Vec<Token>
}

impl Parser {
    fn new (tokens: Vec<Token>) -> Self {
        Self {current:0, tokens}
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn expect(&mut self, kind:TokenKind) -> &Token {
        self.advance();
        let tok = self.tokens.get(self.current-1).unwrap();
        if tok.kind == kind {
            return tok;
        }
        panic!("unexpected token {:?}, expect {:?}", tok.kind, kind);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        if !self.is_at_end() {
            return self.tokens.get(self.current).unwrap()
        }
        panic!("end of file")
    }

    fn check(&self, kind: TokenKind) -> bool {
        if !self.is_at_end() {
            return self.tokens.get(self.current).unwrap().kind == kind
        }
        panic!("end of file")
    }

    fn match_tokens(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if &self.tokens.get(self.current).unwrap().kind == kind {
                self.advance();
                return true;
            }
        }
        false
    }

    // skill issue LOL
    fn value(&mut self) -> JsonValue {
        let tok = self.peek();
        match &tok.kind {
            TokenKind::Lbrace => {
                self.object()
            }
            TokenKind::Lbracket => {
                self.array()
            }
            TokenKind::Boolen => {
                match tok.kind {
                    TokenKind::Boolen => {
                        let val = match &tok.value {
                            Value::Boolen(Boolean::True) => JsonValue::Boolean(true),
                            Value::Boolen(Boolean::False) => JsonValue::Boolean(false),
                            Value::Boolen(Boolean::Null) => JsonValue::Null,
                            v => unreachable!("{:?}", v)
                        };
                        self.advance();
                        val
                    }
                    _ => unreachable!()
                }
            }
            TokenKind::String => {
                let value = JsonValue::String(match &tok.value {
                    Value::String(s) => s.clone(),
                    _ => unreachable!()
                });
                self.advance();
                value
            }
            TokenKind::Number => {
                match &tok.value {
                    Value::Number(n) => {
                        let val = match n {
                            Number::Float(f) => JsonValue::Float(*f),
                            Number::Integer(i) => JsonValue::Integer(*i)
                        };
                        self.advance();
                        val

                    }
                    _=> unreachable!()
                }
            } 
            gabbage => panic!("unexpected token {:?}, expect value", gabbage)
        }
    }

    fn array(&mut self) -> JsonValue {
        self.expect(TokenKind::Lbracket);

        let mut values: Vec<JsonValue> = vec![];

        if self.match_tokens(&[TokenKind::Rbracket]) {
            return JsonValue::Array(values);
        }

        loop {
            if self.check(TokenKind::Rbracket) {
                panic!("trailing ,")
            }

            values.push(self.value());
            //self.expect(TokenKind::Comma);
            if !self.match_tokens(&[TokenKind::Comma]) {
                break;
            }
        }
        self.expect(TokenKind::Rbracket);

        JsonValue::Array(values)
    }

    fn object(&mut self) -> JsonValue {
        self.expect(TokenKind::Lbrace);

        let mut map  = HashMap::new();

        if self.match_tokens(&[TokenKind::Rbrace]) {
            return JsonValue::Object(map)
        }
        loop {
            if self.check(TokenKind::Rbrace) {
                panic!("trailing ,")
            }

            let key = self.expect(TokenKind::String);
            let key = match &key.value {
                Value::String(s) => s.clone(),
                _ => unreachable!()
            };

            self.expect(TokenKind::Colon);
            map.insert(key, self.value());
            if !self.match_tokens(&[TokenKind::Comma]) {
                break;
            }
        }
        self.expect(TokenKind::Rbrace);
        JsonValue::Object(map)
    }

    fn parse(&mut self) -> JsonValue {
        match &self.peek().kind {
            TokenKind::Boolen => self.value(),
            TokenKind::String => self.value(),
            TokenKind::Lbrace => self.object(),
            TokenKind::Lbracket => self.array(),
            TokenKind::Number => self.value(),
            gabbage => panic!("unexpected token {:?}", gabbage)
        }
    }
}

#[derive(Debug)]
enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    Boolean(bool),
    Integer(i32),
    Float(f32),
    String(String),
    Null
}

fn main() {
    let tokens = Lexer::new("{\"\":true, \"one\": [{},{},{},{}] }".into()).lex();
    let value = Parser::new(tokens).parse();
    println!("{:#?}", value)
}
