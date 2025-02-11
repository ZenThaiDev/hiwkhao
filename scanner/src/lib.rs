use grammar::Token;
use logos::Logos;

pub mod grammar;

pub fn tokenize(input: &str) -> logos::Lexer<'_, Token> {
    println!("DEBUG [Scanner]: Starting tokenization of input: {}", input);
    let lexer = grammar::Token::lexer(input);
    lexer
}

pub fn tokenize_vector(input: &str) -> Vec<(String, grammar::Token)> {
    println!("DEBUG [Scanner]: Starting vector tokenization");
    let mut lexer = grammar::Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        let slice = lexer.slice().to_string();
        match token {
            Ok(tok) => {
                println!("DEBUG [Scanner]: Generated token: {:?} from slice: {}", tok, slice);
                tokens.push((slice, tok))
            },
            Err(_) => {
                println!("DEBUG [Scanner]: Error generating token for slice: {}", slice);
                tokens.push((slice, grammar::Token::ERR))
            },
        }
    }

    println!("DEBUG [Scanner]: Completed tokenization, total tokens: {}", tokens.len());
    tokens
}

pub fn run_scanner(input: &str) -> Vec<String> {
    let lines: Vec<&str> = input.lines().collect();
    let mut final_output: Vec<String> = vec![];

    for line in lines {
        let tokens = tokenize_vector(line);
        let formatted_output: Vec<String> = tokens
            .into_iter()
            .map(|(word, token)| {
                let token_name = match token {
                    grammar::Token::REAL(_) => "REAL",
                    grammar::Token::INT(_) => "INT",
                    grammar::Token::VAR(_) => "VAR",
                    grammar::Token::ADD => "+",
                    grammar::Token::SUB => "-",
                    grammar::Token::MUL => "*",
                    grammar::Token::DIV => "/",
                    grammar::Token::INTDIV => "//",
                    grammar::Token::POW => "POW",
                    grammar::Token::LPAREN => "LPAREN",
                    grammar::Token::RPAREN => "RPAREN",
                    grammar::Token::LBRACKET => "LBRACKET",
                    grammar::Token::RBRACKET => "RBRACKET",
                    grammar::Token::EQ => "==",
                    grammar::Token::NE => "!=",
                    grammar::Token::LT => "<",
                    grammar::Token::GT => ">",
                    grammar::Token::LE => "<=",
                    grammar::Token::GE => ">=",
                    grammar::Token::ASSIGN => "=",
                    grammar::Token::LIST => "list",
                    grammar::Token::ERR => "ERR",
                    _ => "UNKNOWN",
                };

                format!("{}/{}", word, token_name)
            })
            .collect();

        final_output.push(formatted_output.join(" "));
    }

    final_output
}
