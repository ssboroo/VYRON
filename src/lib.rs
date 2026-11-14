//! VYRON language front-end foundations.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic { pub message: String, pub offset: usize }

impl Diagnostic {
    fn new(message: impl Into<String>, offset: usize) -> Self {
        Self { message: message.into(), offset }
    }
}
impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at byte {}", self.message, self.offset)
    }
}
impl std::error::Error for Diagnostic {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Ident(String), Number(String), String(String), Fn, Effect, Module, Return,
    Arrow, Bang, LBrace, RBrace, LParen, RParen, Colon, Comma, Semicolon,
    Other(char), Eof,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token { pub kind: TokenKind, pub offset: usize }

pub fn lex(source: &str) -> Result<Vec<Token>, Diagnostic> {
    let chars: Vec<(usize, char)> = source.char_indices().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let (offset, ch) = chars[i];
        if ch.is_whitespace() { i += 1; continue; }
        if ch == '/' && chars.get(i + 1).map(|(_, c)| *c) == Some('/') {
            i += 2;
            while i < chars.len() && chars[i].1 != '\n' { i += 1; }
            continue;
        }
        if ch.is_ascii_alphabetic() || ch == '_' {
            let start = i;
            i += 1;
            while i < chars.len() && (chars[i].1.is_ascii_alphanumeric() || chars[i].1 == '_') { i += 1; }
            let word: String = chars[start..i].iter().map(|(_, c)| *c).collect();
            let kind = match word.as_str() {
                "fn" => TokenKind::Fn, "effect" => TokenKind::Effect,
                "module" => TokenKind::Module, "return" => TokenKind::Return,
                _ => TokenKind::Ident(word),
            };
            tokens.push(Token { kind, offset });
            continue;
        }
        if ch.is_ascii_digit() {
            let start = i;
            i += 1;
            while i < chars.len() && chars[i].1.is_ascii_digit() { i += 1; }
            let value: String = chars[start..i].iter().map(|(_, c)| *c).collect();
            tokens.push(Token { kind: TokenKind::Number(value), offset });
            continue;
        }
        if ch == '"' {
            i += 1;
            let mut value = String::new();
            let mut closed = false;
            while i < chars.len() {
                let current = chars[i].1;
                if current == '"' { closed = true; i += 1; break; }
                if current == '\\' {
                    i += 1;
                    let Some((_, escaped)) = chars.get(i) else {
                        return Err(Diagnostic::new("unterminated string escape", offset));
                    };
                    value.push(match escaped {
                        'n' => '\n', 'r' => '\r', 't' => '\t',
                        '"' => '"', '\\' => '\\', other => *other,
                    });
                    i += 1;
                } else { value.push(current); i += 1; }
            }
            if !closed { return Err(Diagnostic::new("unterminated string literal", offset)); }
            tokens.push(Token { kind: TokenKind::String(value), offset });
            continue;
        }
        let kind = if ch == '-' && chars.get(i + 1).map(|(_, c)| *c) == Some('>') {
            i += 2; TokenKind::Arrow
        } else {
            i += 1;
            match ch {
                '!' => TokenKind::Bang, '{' => TokenKind::LBrace, '}' => TokenKind::RBrace,
                '(' => TokenKind::LParen, ')' => TokenKind::RParen, ':' => TokenKind::Colon,
                ',' => TokenKind::Comma, ';' => TokenKind::Semicolon, other => TokenKind::Other(other),
            }
        };
        tokens.push(Token { kind, offset });
    }
    tokens.push(Token { kind: TokenKind::Eof, offset: source.len() });
    Ok(tokens)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDecl { pub name: String, pub return_type: Option<String>, pub effects: Vec<String> }
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Module { pub name: Option<String>, pub effects: Vec<String>, pub functions: Vec<FunctionDecl> }

pub fn parse(source: &str) -> Result<Module, Diagnostic> {
    Parser { tokens: lex(source)?, cursor: 0 }.parse_module()
}
struct Parser { tokens: Vec<Token>, cursor: usize }
impl Parser {
    fn current(&self) -> &Token { &self.tokens[self.cursor] }
    fn advance(&mut self) -> Token {
        let token = self.tokens[self.cursor].clone();
        if !matches!(token.kind, TokenKind::Eof) { self.cursor += 1; }
        token
    }
    fn expect(&mut self, expected: TokenKind, message: &str) -> Result<Token, Diagnostic> {
        if std::mem::discriminant(&self.current().kind) == std::mem::discriminant(&expected) {
            Ok(self.advance())
        } else { Err(Diagnostic::new(message, self.current().offset)) }
    }
    fn take_ident(&mut self, message: &str) -> Result<String, Diagnostic> {
        match self.advance() {
            Token { kind: TokenKind::Ident(name), .. } => Ok(name),
            token => Err(Diagnostic::new(message, token.offset)),
        }
    }
    fn parse_module(&mut self) -> Result<Module, Diagnostic> {
        let mut module = Module::default();
        if matches!(self.current().kind, TokenKind::Module) {
            self.advance();
            module.name = Some(self.take_ident("expected module name")?);
            self.expect(TokenKind::Semicolon, "expected ';' after module declaration")?;
        }
        while !matches!(self.current().kind, TokenKind::Eof) {
            match self.current().kind {
                TokenKind::Effect => {
                    self.advance();
                    module.effects.push(self.take_ident("expected effect name")?);
                    self.expect(TokenKind::Semicolon, "expected ';' after effect declaration")?;
                }
                TokenKind::Fn => module.functions.push(self.parse_function()?),
                _ => return Err(Diagnostic::new("expected 'effect' or 'fn' declaration", self.current().offset)),
            }
        }
        Ok(module)
    }
    fn parse_function(&mut self) -> Result<FunctionDecl, Diagnostic> {
        self.expect(TokenKind::Fn, "expected 'fn'")?;
        let name = self.take_ident("expected function name")?;
        self.expect(TokenKind::LParen, "expected '(' after function name")?;
        self.expect(TokenKind::RParen, "VYRON 0.1 currently supports zero-argument functions")?;
        let return_type = if matches!(self.current().kind, TokenKind::Arrow) {
            self.advance(); Some(self.take_ident("expected return type")?)
        } else { None };
        let mut effects = Vec::new();
        if matches!(self.current().kind, TokenKind::Bang) {
            self.advance();
            self.expect(TokenKind::LBrace, "expected '{' after effect marker")?;
            if !matches!(self.current().kind, TokenKind::RBrace) {
                loop {
                    effects.push(self.take_ident("expected effect name")?);
                    if matches!(self.current().kind, TokenKind::Comma) { self.advance(); } else { break; }
                }
            }
            self.expect(TokenKind::RBrace, "expected '}' after effect list")?;
        }
        self.expect(TokenKind::LBrace, "expected function body")?;
        let mut depth = 1usize;
        while depth > 0 {
            let token = self.advance();
            match token.kind {
                TokenKind::LBrace => depth += 1,
                TokenKind::RBrace => depth -= 1,
                TokenKind::Eof => return Err(Diagnostic::new("unterminated function body", token.offset)),
                _ => {}
            }
        }
        Ok(FunctionDecl { name, return_type, effects })
    }
}

pub fn check_effects(module: &Module) -> Result<(), Diagnostic> {
    for function in &module.functions {
        for effect in &function.effects {
            if !module.effects.iter().any(|declared| declared == effect) {
                return Err(Diagnostic::new(
                    format!("function '{}' uses undeclared effect '{}'", function.name, effect), 0
                ));
            }
        }
    }
    Ok(())
}
pub fn check(source: &str) -> Result<Module, Diagnostic> {
    let module = parse(source)?;
    check_effects(&module)?;
    Ok(module)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lexes_keywords_and_function_punctuation() {
        let tokens = lex("fn main() -> i64 !{io} {}").unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::Fn));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Arrow)));
        assert!(matches!(tokens.last().unwrap().kind, TokenKind::Eof));
    }
    #[test]
    fn parses_module_and_declared_effects() {
        let module = check("module demo; effect io; fn main() -> i64 !{io} { return 0; }").unwrap();
        assert_eq!(module.name.as_deref(), Some("demo"));
        assert_eq!(module.functions[0].name, "main");
        assert_eq!(module.functions[0].effects, vec!["io"]);
    }
    #[test]
    fn rejects_undeclared_effects() {
        let error = check("fn main() !{network} {}").unwrap_err();
        assert!(error.message.contains("undeclared effect"));
    }
    #[test]
    fn rejects_unterminated_strings() {
        assert!(lex("fn main() { \"oops }").is_err());
    }
    #[test]
    fn rejects_unterminated_function_bodies() {
        assert!(parse("fn main() {").unwrap_err().message.contains("unterminated"));
    }
}
