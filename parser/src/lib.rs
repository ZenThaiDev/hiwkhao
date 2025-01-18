use logos::Lexer;
use scanner_lib::grammar::Token;
use std::collections::HashMap;

pub mod symbol_table;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Variable(String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
    Assignment(String, Box<Expr>),
    Boolean(Box<Expr>, String, Box<Expr>),
    List(Vec<f64>),
    ListAccess(String, Box<Expr>),
    UnaryOp(String, Box<Expr>),
}

impl Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Number(n) => {
                if n.fract() == 0.0 {
                    n.to_string()
                } else {
                    format!("{:.1}", n)
                }
            }
            Expr::Variable(name) => name.clone(),
            Expr::BinaryOp(left, op, right) => {
                format!("({}{}{})", left.to_string(), op, right.to_string())
            }
            Expr::UnaryOp(op, expr) => {
                format!("({}{}", op, expr.to_string() + ")")
            }
            Expr::Assignment(var, expr) => {
                format!("({}={})", var, expr.to_string())
            }
            Expr::Boolean(left, op, right) => {
                format!("({}{}{}", left.to_string(), op, right.to_string() + ")")
            }
            Expr::List(lst) => format!("(list[({})])", lst.len().to_string()),
            Expr::ListAccess(var, idx) => format!("({}[({})])", var, idx.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum ParseResult {
    Success(String),
    Error(ParseError),
}

#[derive(Debug)]
pub enum ParseError {
    SyntaxError(Position),
    UndefinedVariable(String, Position),
    InvalidAtom(Position),
    IndexOutOfRange(Position),
    DivisionByZero(Position),
    TokenizeError,
}
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    variables: HashMap<String, Expr>,
    current_line: usize,
    current_column: usize,
    token_positions: Vec<usize>,
}

pub fn token_length(token: &Token) -> usize {
    match token {
        Token::VAR(name) => name.len(),
        Token::INT(n) => n.len(),
        Token::REAL(n) => n.len(),
        Token::ADD | Token::SUB | Token::MUL | Token::DIV => 1,
        Token::POW => 1,
        Token::LPAREN | Token::RPAREN => 1,
        Token::LBRACKET | Token::RBRACKET => 1,
        Token::ASSIGN => 1,
        Token::EQ => 2,     // ==
        Token::NE => 2,     // !=
        Token::GT => 1,     // >
        Token::LE => 2,     // <=
        Token::GE => 2,     // >=
        Token::LT => 1,     // <
        Token::LIST => 4,   // "list"
        Token::INTDIV => 2, // //
        Token::NEWLINE => 1,
        Token::WHITESPACE => 1,
        Token::ERR => 1,
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
            variables: HashMap::new(),
            current_line: 1,
            current_column: 1,
            token_positions: Vec::new(),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn consume(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let token = self.tokens[self.pos].clone();
            self.pos += 1;

            // Update column based on token length
            self.current_column += token_length(&token);

            // Add space after token if there is one and it's not the last token
            if self.pos < self.tokens.len() {
                // Check if there's whitespace between current token and next token
                // This would need to be provided by your scanner/tokenizer
                self.current_column += 1; // Assuming single space between tokens
            }

            Some(token)
        } else {
            None
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        match self.peek() {
            Some(token) if token == &expected => {
                self.consume();
                Ok(())
            }
            Some(_) => Err(ParseError::SyntaxError(Position {
                line: self.current_line,
                column: self.current_column,
            })),
            _ => Err(ParseError::SyntaxError(Position {
                line: self.current_line,
                column: self.current_column,
            })),
        }
    }

    fn get_current_position(&self) -> Position {
        Position {
            line: self.current_line,
            column: if self.pos > 0 && self.pos <= self.token_positions.len() {
                self.token_positions[self.pos - 1]
            } else if !self.token_positions.is_empty() {
                self.token_positions[0]
            } else {
                self.current_column
            },
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.parse_calculation()
    }

    fn parse_calculation(&mut self) -> Result<Expr, ParseError> {
        match self.peek() {
            Some(Token::VAR(_)) => {
                let next = self.tokens.get(self.pos + 1);
                if next == Some(&Token::ASSIGN) {
                    self.parse_assignment()
                } else {
                    self.parse_boolean()
                }
            }
            _ => self.parse_boolean(),
        }
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        if let Some(Token::VAR(name)) = self.consume() {
            self.expect(Token::ASSIGN)?;
            let expr = self.parse_expression()?;

            match &expr {
                Expr::List(lst) => {
                    if lst.is_empty() {
                        return Err(ParseError::SyntaxError(self.get_current_position()));
                    }
                    self.variables.insert(name.clone(), expr.clone());
                }
                _ => {
                    self.variables.insert(name.clone(), expr.clone());
                }
            }

            Ok(Expr::Assignment(name, Box::new(expr)))
        } else {
            Err(ParseError::SyntaxError(self.get_current_position()))
        }
    }

    fn parse_boolean(&mut self) -> Result<Expr, ParseError> {
        let left = self.parse_expression()?;

        match self.peek() {
            Some(Token::EQ) => {
                self.consume();
                let right = self.parse_expression()?;
                Ok(Expr::Boolean(
                    Box::new(left),
                    "==".to_string(),
                    Box::new(right),
                ))
            }
            Some(Token::NE) => {
                self.consume();
                let right = self.parse_expression()?;
                Ok(Expr::Boolean(
                    Box::new(left),
                    "!=".to_string(),
                    Box::new(right),
                ))
            }
            Some(Token::GT) => {
                self.consume();
                let right = self.parse_expression()?;
                Ok(Expr::Boolean(
                    Box::new(left),
                    ">".to_string(),
                    Box::new(right),
                ))
            }
            _ => Ok(left),
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        let expr = if let Some(Token::SUB) = self.peek() {
            self.consume();
            let term = self.parse_term()?;
            Expr::BinaryOp(
                Box::new(Expr::UnaryOp("-".to_string(), Box::new(term))),
                "+".to_string(),
                Box::new(Expr::Number(0.0)),
            )
        } else {
            self.parse_term()?
        };
    
        let mut result = expr;
    
        while let Some(token) = self.peek().cloned() {
            match token {
                Token::ADD | Token::SUB => {
                    self.consume();
                    // Check if the next token is LIST
                    if let Some(Token::LIST) = self.peek() {
                        return Err(ParseError::SyntaxError(self.get_current_position()));
                    }
                    let right = self.parse_term()?;
                    result = Expr::BinaryOp(
                        Box::new(result),
                        if matches!(token, Token::ADD) { "+" } else { "-" }.to_string(),
                        Box::new(right),
                    );
                }
                _ => break,
            }
        }
        Ok(result)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_factor()?;
    
        while let Some(token) = self.peek().cloned() {
            match token {
                Token::MUL | Token::DIV => {
                    self.consume();
                    // Check if the next token is LIST
                    if let Some(Token::LIST) = self.peek() {
                        return Err(ParseError::SyntaxError(self.get_current_position()));
                    }
                    let right = self.parse_factor()?;
                    left = Expr::BinaryOp(
                        Box::new(left),
                        if matches!(token, Token::MUL) { "*" } else { "/" }.to_string(),
                        Box::new(right),
                    );
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = if let Some(Token::SUB) = self.peek() {
            self.consume();
            let atom = self.parse_atom()?;
            Expr::UnaryOp("-".to_string(), Box::new(atom))
        } else {
            self.parse_atom()?
        };

        while let Some(Token::POW) = self.peek() {
            self.consume();
            let right = self.parse_factor()?;
            expr = Expr::BinaryOp(Box::new(expr), "^".to_string(), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_atom(&mut self) -> Result<Expr, ParseError> {
        match self.peek().cloned() {
            Some(Token::LPAREN) => {
                self.consume();
                let expr = self.parse_expression()?;
                self.expect(Token::RPAREN)?;
                Ok(expr)
            }
            Some(Token::INT(n)) => {
                self.consume();
                if let Some(token) = self.peek() {
                    if !matches!(
                        token,
                        Token::ADD
                            | Token::SUB
                            | Token::MUL
                            | Token::DIV
                            | Token::POW
                            | Token::EQ
                            | Token::NE
                            | Token::GT
                            | Token::LPAREN
                            | Token::RPAREN
                            | Token::RBRACKET
                    ) {
                        return Err(ParseError::SyntaxError(self.get_current_position()));
                    }
                }
                Ok(Expr::Number(n.parse::<f64>().map_err(|_| {
                    ParseError::SyntaxError(self.get_current_position())
                })?))
            }
            Some(Token::REAL(n)) => {
                self.consume();
                if let Some(token) = self.peek() {
                    if !matches!(
                        token,
                        Token::ADD
                            | Token::SUB
                            | Token::MUL
                            | Token::DIV
                            | Token::POW
                            | Token::EQ
                            | Token::NE
                            | Token::GT
                            | Token::LPAREN
                            | Token::RPAREN
                            | Token::RBRACKET
                    ) {
                        return Err(ParseError::SyntaxError(self.get_current_position()));
                    }
                }
                Ok(Expr::Number(n.parse::<f64>().map_err(|_| {
                    ParseError::SyntaxError(self.get_current_position())
                })?))
            }
            Some(Token::VAR(name)) => {
                self.consume();

                // Check if this is a list access
                if let Some(Token::LBRACKET) = self.peek() {
                    self.consume();
                    let index_expr = self.parse_expression()?;
                    self.expect(Token::RBRACKET)?;

                    if !self.variables.contains_key(&name) {
                        return Err(ParseError::UndefinedVariable(
                            name,
                            self.get_current_position(),
                        ));
                    }

                    Ok(Expr::ListAccess(name, Box::new(index_expr)))
                } else {
                    if !self.variables.contains_key(&name) {
                        return Err(ParseError::UndefinedVariable(
                            name,
                            self.get_current_position(),
                        ));
                    }
                    Ok(Expr::Variable(name))
                }
            }
            Some(Token::LIST) => {
                self.consume();
                self.expect(Token::LBRACKET)?;
                let size = if let Some(Token::INT(n)) = self.consume() {
                    n.parse::<usize>()
                        .map_err(|_| ParseError::SyntaxError(self.get_current_position()))?
                } else {
                    return Err(ParseError::SyntaxError(self.get_current_position()));
                };
    
                self.expect(Token::RBRACKET)?;
    
                // Check if list[n] is followed by any operation
                if let Some(token) = self.peek() {
                    match token {
                        Token::ADD | Token::SUB | Token::MUL | Token::DIV | Token::POW => {
                            return Err(ParseError::SyntaxError(self.get_current_position()));
                        }
                        _ => {}
                    }
                }
    
                if size == 0 {
                    return Err(ParseError::SyntaxError(self.get_current_position()));
                }
    
                Ok(Expr::List(vec![0.0; size]))
            }
            Some(Token::ERR) => Err(ParseError::SyntaxError(self.get_current_position())),

            _ => Err(ParseError::InvalidAtom(self.get_current_position())),
        }
    }

    pub fn parse_tokens(&mut self, input: Lexer<'_, Token>) -> ParseResult {
        let tokens = input.collect::<Vec<_>>();

        // Split tokens into lines and track positions
        let mut current_line = 1;
        let mut lines: Vec<(Vec<Token>, Vec<usize>)> = Vec::new();
        let mut current_line_tokens = Vec::new();
        let mut current_line_positions = Vec::new();
        let mut column = 1;

        for token in tokens {
            match &token {
                Ok(Token::NEWLINE) => {
                    if !current_line_tokens.is_empty() {
                        lines.push((current_line_tokens, current_line_positions));
                        current_line_tokens = Vec::new();
                        current_line_positions = Vec::new();
                    }
                    column = 1;
                }
                Ok(Token::WHITESPACE) => {
                    column += 1;
                }
                _ => {
                    if let Ok(tok) = token {
                        current_line_positions.push(column);
                        current_line_tokens.push(tok);
                        column += token_length(&current_line_tokens.last().unwrap());
                    } else {
                        current_line_positions.push(column);
                        current_line_tokens.push(Token::ERR);
                        column += token_length(&current_line_tokens.last().unwrap());
                    }
                }
            }
        }

        // Add the last line if it doesn't end with a newline
        if !current_line_tokens.is_empty() {
            lines.push((current_line_tokens, current_line_positions));
        }

        // Process each line
        for (_line_num, (line_tokens, positions)) in lines.iter().enumerate() {
            self.tokens = line_tokens.clone();
            self.token_positions = positions.clone();
            self.pos = 0;
            self.current_line = current_line;
            self.current_column = 1;

            match self.parse() {
                Ok(expr) => {
                    println!("{}", expr.to_string());
                    current_line += 1;
                }
                Err(err) => {
                    match err {
                        ParseError::UndefinedVariable(var, pos) => {
                            println!(
                                "Undefined variable {} at line {}, pos {}",
                                var, pos.line, pos.column
                            );
                        }
                        ParseError::SyntaxError(pos) => {
                            println!("SyntaxError at line {}, pos {}", pos.line, pos.column);
                        }
                        ParseError::InvalidAtom(pos) => {
                            println!("Invalid atom at line {}, pos {}", pos.line, pos.column);
                        }
                        ParseError::IndexOutOfRange(pos) => {
                            println!("IndexOutOfRange at line {}, pos {}", pos.line, pos.column);
                        }
                        ParseError::DivisionByZero(pos) => {
                            println!("Division by zero at line {}, pos {}", pos.line, pos.column);
                        }
                        ParseError::TokenizeError => {
                            println!("TokenizeError");
                        }
                    }
                    current_line += 1;
                }
            }
        }

        ParseResult::Success("Parsing completed".to_string())
    }

    pub fn parse_tokens_fancy(&mut self, input: Lexer<'_, Token>) -> Vec<String> {
        let tokens = input.collect::<Vec<_>>();

        // Split tokens into lines and track positions
        let mut current_line = 1;
        let mut lines: Vec<(Vec<Token>, Vec<usize>)> = Vec::new();
        let mut current_line_tokens = Vec::new();
        let mut current_line_positions = Vec::new();
        let mut column = 1;

        let mut output = Vec::new();

        for token in tokens {
            match &token {
                Ok(Token::NEWLINE) => {
                    if !current_line_tokens.is_empty() {
                        lines.push((current_line_tokens, current_line_positions));
                        current_line_tokens = Vec::new();
                        current_line_positions = Vec::new();
                    }
                    column = 1;
                }
                Ok(Token::WHITESPACE) => {
                    column += 1;
                }
                _ => {
                    if let Ok(tok) = token {
                        current_line_positions.push(column);
                        current_line_tokens.push(tok);
                        column += token_length(&current_line_tokens.last().unwrap());
                    } else {
                        current_line_positions.push(column);
                        current_line_tokens.push(Token::ERR);
                        column += token_length(&current_line_tokens.last().unwrap());
                    }
                }
            }
        }

        // Add the last line if it doesn't end with a newline
        if !current_line_tokens.is_empty() {
            lines.push((current_line_tokens, current_line_positions));
        }

        // Process each line
        for (_line_num, (line_tokens, positions)) in lines.iter().enumerate() {
            self.tokens = line_tokens.clone();
            self.token_positions = positions.clone();
            self.pos = 0;
            self.current_line = current_line;
            self.current_column = 1;

            match self.parse() {
                Ok(expr) => {
                    output.push(expr.to_string());
                    current_line += 1;
                }
                Err(err) => {
                    match err {
                        ParseError::UndefinedVariable(var, pos) => {
                            output.push(format!(
                                "Undefined variable {} at line {}, pos {}",
                                var, pos.line, pos.column
                            ));
                        }
                        ParseError::SyntaxError(pos) => {
                            output.push(format!(
                                "SyntaxError at line {}, pos {}",
                                pos.line, pos.column
                            ));
                        }
                        ParseError::InvalidAtom(pos) => {
                            output.push(format!(
                                "Invalid atom at line {}, pos {}",
                                pos.line, pos.column
                            ));
                        }
                        ParseError::IndexOutOfRange(pos) => {
                            output.push(format!(
                                "IndexOutOfRange at line {}, pos {}",
                                pos.line, pos.column
                            ));
                        }
                        ParseError::DivisionByZero(pos) => {
                            output.push(format!(
                                "Division by zero at line {}, pos {}",
                                pos.line, pos.column
                            ));
                        }
                        ParseError::TokenizeError => {
                            output.push("TokenizeError".to_string());
                        }
                    }
                    current_line += 1;
                }
            }
        }

        output
    }
}
