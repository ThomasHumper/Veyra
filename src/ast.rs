src/main.rs
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(i64),
    Identifier(String),

    Plus,
    Minus,
    Star,
    Slash,

    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,

    Let,
    If,
    Else,
    Fun,
    Return,
    Print,

    True,
    False,

    Eof,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += 1;
        Some(ch)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn lex_number(&mut self, first: char) -> Token {
        let mut value = first.to_string();

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        Token::Number(value.parse().unwrap())
    }

    fn lex_identifier(&mut self, first: char) -> Token {
        let mut value = first.to_string();

        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        match value.as_str() {
            "lum" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "fun" => Token::Fun,
            "return" => Token::Return,
            "print" => Token::Print,
            "true" => Token::True,
            "false" => Token::False,

            // Veyra number words.
            "ka" => Token::Number(1),
            "ve" => Token::Number(2),
            "tri" => Token::Number(3),
            "nox" => Token::Number(4),
            "sai" => Token::Number(5),
            "lum6" => Token::Number(6),
            "dra" => Token::Number(7),
            "kei" => Token::Number(8),
            "vor" => Token::Number(9),
            "zen" => Token::Number(10),

            _ => Token::Identifier(value),
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.advance() {
            match ch {
                ' ' | '\t' | '\r' | '\n' => {}

                '/' if self.peek() == Some('/') => {
                    while let Some(next) = self.peek() {
                        self.advance();

                        if next == '\n' {
                            break;
                        }
                    }
                }

                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Star),
                '/' => tokens.push(Token::Slash),

                '=' => {
                    if self.match_char('=') {
                        tokens.push(Token::EqualEqual);
                    } else {
                        tokens.push(Token::Equal);
                    }
                }

                '!' => {
                    if self.match_char('=') {
                        tokens.push(Token::BangEqual);
                    } else {
                        return Err("Unexpected `!`".into());
                    }
                }

                '<' => {
                    if self.match_char('=') {
                        tokens.push(Token::LessEqual);
                    } else {
                        tokens.push(Token::Less);
                    }
                }

                '>' => {
                    if self.match_char('=') {
                        tokens.push(Token::GreaterEqual);
                    } else {
                        tokens.push(Token::Greater);
                    }
                }

                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                '{' => tokens.push(Token::LeftBrace),
                '}' => tokens.push(Token::RightBrace),
                ',' => tokens.push(Token::Comma),
                ';' => tokens.push(Token::Semicolon),

                ch if ch.is_ascii_digit() => {
                    tokens.push(self.lex_number(ch));
                }

                ch if ch.is_ascii_alphabetic() || ch == '_' => {
                    tokens.push(self.lex_identifier(ch));
                }

                _ => {
                    return Err(format!("Unexpected character `{ch}`"));
                }
            }
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }
}

#[derive(Debug, Clone)]
enum Expr {
    Number(i64),
    Bool(bool),
    Variable(String),

    Unary {
        op: Token,
        right: Box<Expr>,
    },

    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },

    Call {
        name: String,
        arguments: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
enum Stmt {
    Expression(Expr),

    Let {
        name: String,
        initializer: Expr,
    },

    Block(Vec<Stmt>),

    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

    Function {
        name: String,
        parameters: Vec<String>,
        body: Vec<Stmt>,
    },

    Return(Option<Expr>),

    Print(Expr),
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.current].clone();

        if self.current < self.tokens.len() - 1 {
            self.current += 1;
        }

        token
    }

    fn check(&self, token: &Token) -> bool {
        self.peek() == token
    }

    fn matches(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, token: &Token, message: &str) -> Result<(), String> {
        if self.matches(token) {
            Ok(())
        } else {
            Err(format!("{message}; found {:?}", self.peek()))
        }
    }

    fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();

        while !self.check(&Token::Eof) {
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.matches(&Token::Let) {
            return self.let_statement();
        }

        if self.matches(&Token::If) {
            return self.if_statement();
        }

        if self.matches(&Token::Fun) {
            return self.function_statement();
        }

        if self.matches(&Token::Return) {
            let value = if self.check(&Token::Semicolon) {
                None
            } else {
                Some(self.expression()?)
            };

            self.consume(&Token::Semicolon, "Expected `;` after return")?;

            return Ok(Stmt::Return(value));
        }

        if self.matches(&Token::Print) {
            let value = self.expression()?;

            self.consume(&Token::Semicolon, "Expected `;` after print")?;

            return Ok(Stmt::Print(value));
        }

        if self.matches(&Token::LeftBrace) {
            return Ok(Stmt::Block(self.block()?));
        }

        let expression = self.expression()?;

        self.consume(
            &Token::Semicolon,
            "Expected `;` after expression",
        )?;

        Ok(Stmt::Expression(expression))
    }

    fn let_statement(&mut self) -> Result<Stmt, String> {
        let name = match self.advance() {
            Token::Identifier(name) => name,
            other => {
                return Err(format!(
                    "Expected variable name, found {other:?}"
                ))
            }
        };

        self.consume(&Token::Equal, "Expected `=`")?;

        let initializer = self.expression()?;

        self.consume(
            &Token::Semicolon,
            "Expected `;` after variable declaration",
        )?;

        Ok(Stmt::Let {
            name,
            initializer,
        })
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        let condition = self.expression()?;

        self.consume(&Token::LeftBrace, "Expected `{` after condition")?;

        let then_branch = Stmt::Block(self.block()?);

        let else_branch = if self.matches(&Token::Else) {
            self.consume(&Token::LeftBrace, "Expected `{` after else")?;

            Some(Box::new(Stmt::Block(self.block()?)))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn function_statement(&mut self) -> Result<Stmt, String> {
        let name = match self.advance() {
            Token::Identifier(name) => name,
            other => {
                return Err(format!(
                    "Expected function name, found {other:?}"
                ))
            }
        };

        self.consume(&Token::LeftParen, "Expected `(`")?;

        let mut parameters = Vec::new();

        if !self.check(&Token::RightParen) {
            loop {
                match self.advance() {
                    Token::Identifier(name) => parameters.push(name),
                    other => {
                        return Err(format!(
                            "Expected parameter, found {other:?}"
                        ))
                    }
                }

                if !self.matches(&Token::Comma) {
                    break;
                }
            }
        }

        self.consume(&Token::RightParen, "Expected `)`")?;
        self.consume(&Token::LeftBrace, "Expected `{`")?;

        let body = self.block()?;

        Ok(Stmt::Function {
            name,
            parameters,
            body,
        })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();

        while !self.check(&Token::RightBrace)
            && !self.check(&Token::Eof)
        {
            statements.push(self.statement()?);
        }

        self.consume(&Token::RightBrace, "Expected `}`")?;

        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.matches(&Token::EqualEqual)
            || self.matches(&Token::BangEqual)
        {
            let op = self.previous().clone();
            let right = self.comparison()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        loop {
            let op = if self.matches(&Token::Less)
                || self.matches(&Token::LessEqual)
                || self.matches(&Token::Greater)
                || self.matches(&Token::GreaterEqual)
            {
                Some(self.previous().clone())
            } else {
                None
            };

            match op {
                Some(op) => {
                    let right = self.term()?;

                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op,
                        right: Box::new(right),
                    };
                }

                None => break,
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        loop {
            let op = if self.matches(&Token::Plus)
                || self.matches(&Token::Minus)
            {
                Some(self.previous().clone())
            } else {
                None
            };

            match op {
                Some(op) => {
                    let right = self.factor()?;

                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op,
                        right: Box::new(right),
                    };
                }

                None => break,
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        loop {
            let op = if self.matches(&Token::Star)
                || self.matches(&Token::Slash)
            {
                Some(self.previous().clone())
            } else {
                None
            };

            match op {
                Some(op) => {
                    let right = self.unary()?;

                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op,
                        right: Box::new(right),
                    };
                }

                None => break,
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.matches(&Token::Minus) {
            let op = self.previous().clone();

            return Ok(Expr::Unary {
                op,
                right: Box::new(self.unary()?),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::Number(n) => Ok(Expr::Number(n)),

            Token::True => Ok(Expr::Bool(true)),

            Token::False => Ok(Expr::Bool(false)),

            Token::Identifier(name) => {
                if self.matches(&Token::LeftParen) {
                    let mut arguments = Vec::new();

                    if !self.check(&Token::RightParen) {
                        loop {
                            arguments.push(self.expression()?);

                            if !self.matches(&Token::Comma) {
                                break;
                            }
                        }
                    }

                    self.consume(
                        &Token::RightParen,
                        "Expected `)` after arguments",
                    )?;

                    Ok(Expr::Call { name, arguments })
                } else {
                    Ok(Expr::Variable(name))
                }
            }

            Token::LeftParen => {
                let expr = self.expression()?;

                self.consume(
                    &Token::RightParen,
                    "Expected `)`",
                )?;

                Ok(expr)
            }

            other => Err(format!("Expected expression, found {other:?}")),
        }
    }
}

#[derive(Debug, Clone)]
enum Value {
    Number(i64),
    Bool(bool),
    Function(Function),
    Null,
}

#[derive(Debug, Clone)]
struct Function {
    parameters: Vec<String>,
    body: Vec<Stmt>,
}

struct Environment {
    values: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }

    fn child(&self) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(Box::new(self.clone())),
        }
    }

    fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    fn get(&self, name: &str) -> Option<Value> {
        self.values
            .get(name)
            .cloned()
            .or_else(|| self.parent.as_ref()?.get(name))
    }
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            parent: self.parent.clone(),
        }
    }
}

struct Interpreter {
    globals: Environment,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            globals: Environment::new(),
        }
    }

    fn execute(&mut self, statements: &[Stmt]) -> Result<(), String> {
        for statement in statements {
            self.execute_statement(statement)?;
        }

        Ok(())
    }

    fn execute_statement(
        &mut self,
        statement: &Stmt,
    ) -> Result<Option<Value>, String> {
        match statement {
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(None)
            }

            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                println!("{}", display_value(&value));
                Ok(None)
            }

            Stmt::Let {
                name,
                initializer,
            } => {
                let value = self.evaluate(initializer)?;

                self.globals.define(name.clone(), value);

                Ok(None)
            }

            Stmt::Block(statements) => {
                for statement in statements {
                    if let Some(value) =
                        self.execute_statement(statement)?
                    {
                        return Ok(Some(value));
                    }
                }

                Ok(None)
            }

            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = self.evaluate(condition)?;

                if is_truthy(&condition) {
                    self.execute_statement(then_branch)
                } else if let Some(branch) = else_branch {
                    self.execute_statement(branch)
                } else {
                    Ok(None)
                }
            }

            Stmt::Function {
                name,
                parameters,
                body,
            } => {
                let function = Function {
                    parameters: parameters.clone(),
                    body: body.clone(),
                };

                self.globals
                    .define(name.clone(), Value::Function(function));

                Ok(None)
            }

            Stmt::Return(value) => {
                let value = match value {
                    Some(expr) => self.evaluate(expr)?,
                    None => Value::Null,
                };

                Ok(Some(value))
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),

            Expr::Bool(value) => Ok(Value::Bool(*value)),

            Expr::Variable(name) => self
                .globals
                .get(name)
                .ok_or_else(|| format!("Unknown variable `{name}`")),

            Expr::Unary { op, right } => {
                let value = self.evaluate(right)?;

                match (op, value) {
                    (Token::Minus, Value::Number(n)) => {
                        Ok(Value::Number(-n))
                    }

                    _ => Err("Invalid unary operation".into()),
                }
            }

            Expr::Binary { left, op, right } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                self.binary(left, op, right)
            }

            Expr::Call { name, arguments } => {
                let function = self
                    .globals
                    .get(name)
                    .ok_or_else(|| {
                        format!("Unknown function `{name}`")
                    })?;

                let Value::Function(function) = function else {
                    return Err(format!("`{name}` is not a function"));
                };

                if arguments.len() != function.parameters.len() {
                    return Err(format!(
                        "Function `{name}` expects {} arguments, got {}",
                        function.parameters.len(),
                        arguments.len()
                    ));
                }

                let values = arguments
                    .iter()
                    .map(|arg| self.evaluate(arg))
                    .collect::<Result<Vec<_>, _>>()?;

                let mut local = Environment::new();

                for (parameter, value) in
                    function.parameters.iter().zip(values)
                {
                    local.define(parameter.clone(), value);
                }

                let previous = std::mem::replace(
                    &mut self.globals,
                    local,
                );

                let result = self.execute(&function.body);

                self.globals = previous;

                match result? {
                    Some(value) => Ok(value),
                    None => Ok(Value::Null),
                }
            }
        }
    }

    fn binary(
        &self,
        left: Value,
        op: &Token,
        right: Value,
    ) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                match op {
                    Token::Plus => Ok(Value::Number(a + b)),
                    Token::Minus => Ok(Value::Number(a - b)),
                    Token::Star => Ok(Value::Number(a * b)),

                    Token::Slash => {
                        if b == 0 {
                            Err("Division by zero. The revolution has stalled.".into())
                        } else {
                            Ok(Value::Number(a / b))
                        }
                    }

                    Token::Less => Ok(Value::Bool(a < b)),
                    Token::LessEqual
