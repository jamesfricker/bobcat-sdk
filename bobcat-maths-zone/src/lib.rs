//! Proc macro helpers for working with `bobcat-maths` using Python-style integer snippets.
//! Accepts small arithmetic strings and turns them into `bobcat_maths::U` constants.

use num_bigint::{BigInt, Sign};
use num_integer::Integer;
use num_traits::{Signed, ToPrimitive, Zero};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Evaluate a Python-style integer expression at compile time.
///
/// Supported:
/// - Decimal, hex (`0x`/`0X`), binary (`0b`/`0B`), octal (`0o`/`0O`) literals
/// - Underscores in literals (`1_000_000`)
/// - Operators: +, -, *, //, %, **, <<, >>, &, |, ^
/// - Unary operators and parentheses
///
/// Notes:
/// - `/` is not supported (use `//`).
/// - Expression must evaluate to a non-negative integer fitting in 256 bits.
#[proc_macro]
pub fn maths_zone(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as LitStr);
    match expand(expr) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand(expr: LitStr) -> Result<proc_macro2::TokenStream, syn::Error> {
    let value = evaluate_expression(&expr.value())
        .map_err(|msg| syn::Error::new(expr.span(), msg))?;
    let bytes = bigint_to_be_bytes(&value)
        .map_err(|msg| syn::Error::new(expr.span(), msg))?;
    let rendered = bytes.iter().map(|b| quote! { #b });
    Ok(quote! {{
        ::bobcat_maths::U([#(#rendered),*])
    }})
}

fn bigint_to_be_bytes(value: &BigInt) -> Result<[u8; 32], String> {
    match value.sign() {
        Sign::Minus => return Err("result is negative; use a signed representation instead".into()),
        Sign::NoSign | Sign::Plus => {}
    }

    let big = value
        .to_biguint()
        .ok_or_else(|| "failed to convert to unsigned representation".to_string())?;
    if big.bits() > 256 {
        return Err("value does not fit in 256 bits".into());
    }

    let mut out = [0u8; 32];
    let bytes = big.to_bytes_be();
    let offset = 32 - bytes.len();
    out[offset..].copy_from_slice(&bytes);
    Ok(out)
}

fn evaluate_expression(src: &str) -> Result<BigInt, String> {
    let mut tokens = lex(src)?;
    tokens.push(Token::Eof);
    let mut parser = Parser { tokens, pos: 0 };
    let expr = parser.parse_expression()?;
    let trailing = parser.peek();
    if !matches!(trailing, Token::Eof) {
        return Err(format!("unexpected trailing input: {:?}", trailing));
    }
    eval(&expr)
}

fn eval(expr: &Expr) -> Result<BigInt, String> {
    match expr {
        Expr::Number(n) => Ok(n.clone()),
        Expr::Unary(UnaryOp::Plus, inner) => eval(inner),
        Expr::Unary(UnaryOp::Minus, inner) => Ok(-eval(inner)?),
        Expr::Binary(lhs, op, rhs) => {
            let l = eval(lhs)?;
            let r = eval(rhs)?;
            match op {
                BinaryOp::Add => Ok(l + r),
                BinaryOp::Sub => Ok(l - r),
                BinaryOp::Mul => Ok(l * r),
                BinaryOp::FloorDiv => {
                    if r.is_zero() {
                        Err("division by zero".into())
                    } else {
                        Ok(l.div_floor(&r))
                    }
                }
                BinaryOp::Mod => {
                    if r.is_zero() {
                        Err("modulo by zero".into())
                    } else {
                        Ok(l.mod_floor(&r))
                    }
                }
                BinaryOp::Pow => {
                    if r.is_negative() {
                        return Err("negative exponents are not supported".into());
                    }
                    let pow = r
                        .to_u32()
                        .ok_or_else(|| "exponent must be a non-negative 32-bit integer".to_string())?;
                    Ok(l.pow(pow))
                }
                BinaryOp::Shl => {
                    let shift = r
                        .to_usize()
                        .ok_or_else(|| "shift count must be a non-negative integer".to_string())?;
                    Ok(l << shift)
                }
                BinaryOp::Shr => {
                    let shift = r
                        .to_usize()
                        .ok_or_else(|| "shift count must be a non-negative integer".to_string())?;
                    Ok(l >> shift)
                }
                BinaryOp::BitAnd => Ok(l & r),
                BinaryOp::BitOr => Ok(l | r),
                BinaryOp::BitXor => Ok(l ^ r),
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Number(BigInt),
    Plus,
    Minus,
    Star,
    DoubleStar,
    DoubleSlash,
    Percent,
    LParen,
    RParen,
    LtLt,
    GtGt,
    Amp,
    Pipe,
    Caret,
    Eof,
}

fn lex(src: &str) -> Result<Vec<Token>, String> {
    let mut chars = src.chars().peekable();
    let mut tokens = Vec::new();
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            '0'..='9' => tokens.push(read_number(&mut chars)?),
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                if matches!(chars.peek(), Some('*')) {
                    chars.next();
                    tokens.push(Token::DoubleStar);
                } else {
                    tokens.push(Token::Star);
                }
            }
            '/' => {
                chars.next();
                if matches!(chars.peek(), Some('/')) {
                    chars.next();
                    tokens.push(Token::DoubleSlash);
                } else {
                    return Err("`/` is not supported; use `//` for integer floor-division".into());
                }
            }
            '%' => {
                chars.next();
                tokens.push(Token::Percent);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            '<' => {
                chars.next();
                if matches!(chars.peek(), Some('<')) {
                    chars.next();
                    tokens.push(Token::LtLt);
                } else {
                    return Err("expected '<' after '<' for a shift expression".into());
                }
            }
            '>' => {
                chars.next();
                if matches!(chars.peek(), Some('>')) {
                    chars.next();
                    tokens.push(Token::GtGt);
                } else {
                    return Err("expected '>' after '>' for a shift expression".into());
                }
            }
            '&' => {
                chars.next();
                tokens.push(Token::Amp);
            }
            '|' => {
                chars.next();
                tokens.push(Token::Pipe);
            }
            '^' => {
                chars.next();
                tokens.push(Token::Caret);
            }
            _ => {
                return Err(format!("unsupported character '{c}'"));
            }
        }
    }
    Ok(tokens)
}

fn read_number<I>(chars: &mut core::iter::Peekable<I>) -> Result<Token, String>
where
    I: Iterator<Item = char>,
{
    let mut buf = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_hexdigit()
            || c == '_'
            || c == 'x'
            || c == 'X'
            || c == 'b'
            || c == 'B'
            || c == 'o'
            || c == 'O'
        {
            buf.push(c);
            chars.next();
        } else {
            break;
        }
    }

    let parsed = parse_number_literal(&buf)?;
    Ok(Token::Number(parsed))
}

fn parse_number_literal(s: &str) -> Result<BigInt, String> {
    let cleaned: String = s.chars().filter(|c| *c != '_').collect();

    let (digits, radix) = if cleaned.starts_with("0x") || cleaned.starts_with("0X") {
        (&cleaned[2..], 16)
    } else if cleaned.starts_with("0b") || cleaned.starts_with("0B") {
        (&cleaned[2..], 2)
    } else if cleaned.starts_with("0o") || cleaned.starts_with("0O") {
        (&cleaned[2..], 8)
    } else {
        (cleaned.as_str(), 10)
    };

    BigInt::parse_bytes(digits.as_bytes(), radix).ok_or_else(|| match radix {
        16 => "invalid hex literal".into(),
        2 => "invalid binary literal".into(),
        8 => "invalid octal literal".into(),
        _ => "invalid decimal literal".into(),
    })
}

#[derive(Debug)]
enum Expr {
    Number(BigInt),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
}

#[derive(Debug, Copy, Clone)]
enum UnaryOp {
    Plus,
    Minus,
}

#[derive(Debug, Copy, Clone)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    FloorDiv,
    Mod,
    Pow,
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor,
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_bit_or()
    }

    fn parse_bit_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_bit_xor()?;
        while matches!(self.peek(), Token::Pipe) {
            self.next();
            let rhs = self.parse_bit_xor()?;
            expr = Expr::Binary(Box::new(expr), BinaryOp::BitOr, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_bit_xor(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_bit_and()?;
        while matches!(self.peek(), Token::Caret) {
            self.next();
            let rhs = self.parse_bit_and()?;
            expr = Expr::Binary(Box::new(expr), BinaryOp::BitXor, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_bit_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_shift()?;
        while matches!(self.peek(), Token::Amp) {
            self.next();
            let rhs = self.parse_shift()?;
            expr = Expr::Binary(Box::new(expr), BinaryOp::BitAnd, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_shift(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_add_sub()?;
        loop {
            match self.peek() {
                Token::LtLt => {
                    self.next();
                    let rhs = self.parse_add_sub()?;
                    expr = Expr::Binary(Box::new(expr), BinaryOp::Shl, Box::new(rhs));
                }
                Token::GtGt => {
                    self.next();
                    let rhs = self.parse_add_sub()?;
                    expr = Expr::Binary(Box::new(expr), BinaryOp::Shr, Box::new(rhs));
                }
                _ => return Ok(expr),
            }
        }
    }

    fn parse_add_sub(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_mul_div()?;
        loop {
            match self.peek() {
                Token::Plus => {
                    self.next();
                    let rhs = self.parse_mul_div()?;
                    expr = Expr::Binary(Box::new(expr), BinaryOp::Add, Box::new(rhs));
                }
                Token::Minus => {
                    self.next();
                    let rhs = self.parse_mul_div()?;
                    expr = Expr::Binary(Box::new(expr), BinaryOp::Sub, Box::new(rhs));
                }
                _ => return Ok(expr),
            }
        }
    }

    fn parse_mul_div(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_power()?;
        loop {
            match self.peek() {
                Token::Star => {
                    self.next();
                    let rhs = self.parse_power()?;
                    expr = Expr::Binary(Box::new(expr), BinaryOp::Mul, Box::new(rhs));
                }
                Token::DoubleSlash => {
                    self.next();
                    let rhs = self.parse_power()?;
                    expr = Expr::Binary(Box::new(expr), BinaryOp::FloorDiv, Box::new(rhs));
                }
                Token::Percent => {
                    self.next();
                    let rhs = self.parse_power()?;
                    expr = Expr::Binary(Box::new(expr), BinaryOp::Mod, Box::new(rhs));
                }
                _ => return Ok(expr),
            }
        }
    }

    fn parse_power(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary()?;
        if matches!(self.peek(), Token::DoubleStar) {
            self.next();
            let rhs = self.parse_power()?;
            expr = Expr::Binary(Box::new(expr), BinaryOp::Pow, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Token::Plus => {
                self.next();
                Ok(Expr::Unary(UnaryOp::Plus, Box::new(self.parse_unary()?)))
            }
            Token::Minus => {
                self.next();
                Ok(Expr::Unary(UnaryOp::Minus, Box::new(self.parse_unary()?)))
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.next() {
            Token::Number(n) => Ok(Expr::Number(n)),
            Token::LParen => {
                let expr = self.parse_expression()?;
                match self.next() {
                    Token::RParen => Ok(expr),
                    _ => Err("expected ')'".into()),
                }
            }
            Token::Eof => Err("unexpected end of expression".into()),
            other => Err(format!(
                "unexpected token in primary expression: {:?}",
                other
            )),
        }
    }

    fn peek(&self) -> Token {
        self.tokens
            .get(self.pos)
            .cloned()
            .unwrap_or(Token::Eof)
    }

    fn next(&mut self) -> Token {
        let tok = self.peek();
        self.pos += 1;
        tok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_bytes(expr: &str) -> [u8; 32] {
        let value = evaluate_expression(expr).expect("expression should parse");
        bigint_to_be_bytes(&value).expect("expression should fit")
    }

    #[test]
    fn parses_simple_expression() {
        assert_eq!(
            eval_bytes("2**8 + 5"),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 5]
        );
    }

    #[test]
    fn handles_floor_div_and_mod() {
        assert_eq!(
            eval_bytes("((5 // 2) << 4) | (11 % 3)"),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34]
        );
    }

    #[test]
    fn parses_hex_and_binary() {
        assert_eq!(
            eval_bytes("0xff + 0b1"),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]
        );
    }

    #[test]
    fn rejects_negative_results() {
        let err = evaluate_expression("-1")
            .and_then(|v| bigint_to_be_bytes(&v))
            .unwrap_err();
        assert!(err.contains("negative"), "got: {}", err);
    }

    #[test]
    fn rejects_single_slash() {
        let err = evaluate_expression("1/2").unwrap_err();
        assert!(
            err.contains("not supported") && err.contains("//"),
            "got: {}",
            err
        );
    }

    #[test]
    fn parses_uppercase_prefixes_and_underscores() {
        assert_eq!(eval_bytes("0Xff + 0B1_0"), eval_bytes("255 + 2"));
        assert_eq!(eval_bytes("0O7_7"), eval_bytes("63"));
    }
}
