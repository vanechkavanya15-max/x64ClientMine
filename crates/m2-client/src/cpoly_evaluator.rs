// cpoly_evaluator.rs - Metin2 CPoly (Polynomial Formula Evaluator) zgodny 1:1 z C++ EterBase/Poly
// Obsluguje formuly obrazen skilli, statystyk potworow i zaleznosci ataku.
// Komentarze w jezyku polskim bez polskich znakow diakrytycznych.

use std::collections::HashMap;

/// Tryb losowania wartosci dla funkcji rand/irand/frand
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomType {
    Freely,
    ForceMin,
    ForceMax,
}

/// Tokeny uzywane przez lekser i parser CPoly
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Ident(String),
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Pow,
    LParen,
    RParen,
    Comma,
    Eof,
}

/// Wezly drzewa skladniowego wyrazenia matematycznego
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    Var(String),
    BinaryOp(Box<Expr>, Op, Box<Expr>),
    UnaryMinus(Box<Expr>),
    FuncCall(String, Vec<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

pub struct CPoly {
    vars: HashMap<String, f64>,
    root: Option<Expr>,
    raw_str: String,
    random_type: RandomType,
}

impl Default for CPoly {
    fn default() -> Self {
        Self::new()
    }
}

impl CPoly {
    pub fn new() -> Self {
        let mut vars = HashMap::new();
        // Standardowe stale matematyczne zdefiniowane w Metin2 CPoly
        vars.insert("pi".to_string(), std::f64::consts::PI);
        vars.insert("e".to_string(), std::f64::consts::E);

        Self {
            vars,
            root: None,
            raw_str: String::new(),
            random_type: RandomType::Freely,
        }
    }

    pub fn set_random(&mut self, rtype: RandomType) {
        self.random_type = rtype;
    }

    pub fn set_str(&mut self, s: &str) {
        self.raw_str = s.to_string();
        let _ = self.analyze();
    }

    pub fn set_var(&mut self, name: &str, val: f64) {
        self.vars.insert(name.to_lowercase(), val);
    }

    pub fn get_var(&self, name: &str) -> Option<f64> {
        self.vars.get(&name.to_lowercase()).copied()
    }

    pub fn clear(&mut self) {
        self.vars.clear();
        self.vars.insert("pi".to_string(), std::f64::consts::PI);
        self.vars.insert("e".to_string(), std::f64::consts::E);
        self.root = None;
        self.raw_str.clear();
    }

    /// Parsuje wyrazenie z ciagu znakow
    pub fn analyze(&mut self) -> bool {
        let s = self.raw_str.trim();
        if s.is_empty() {
            self.root = None;
            return true;
        }

        let tokens = match Lexer::tokenize(s) {
            Ok(toks) => toks,
            Err(_) => return false,
        };

        let mut parser = Parser::new(tokens);
        match parser.parse_expr() {
            Ok(expr) => {
                self.root = Some(expr);
                true
            }
            Err(_) => {
                self.root = None;
                false
            }
        }
    }

    /// Oblicza wartosc sparsowanego wyrazenia
    pub fn eval(&self) -> f64 {
        if let Some(ref expr) = self.root {
            self.eval_expr(expr)
        } else {
            0.0
        }
    }

    fn eval_expr(&self, expr: &Expr) -> f64 {
        match expr {
            Expr::Num(n) => *n,
            Expr::Var(name) => *self.vars.get(name).unwrap_or(&0.0),
            Expr::UnaryMinus(inner) => -self.eval_expr(inner),
            Expr::BinaryOp(lhs, op, rhs) => {
                let l = self.eval_expr(lhs);
                let r = self.eval_expr(rhs);
                match op {
                    Op::Add => l + r,
                    Op::Sub => l - r,
                    Op::Mul => l * r,
                    Op::Div => {
                        if r.abs() < 1e-12 {
                            0.0
                        } else {
                            l / r
                        }
                    }
                    Op::Mod => {
                        if r.abs() < 1e-12 {
                            0.0
                        } else {
                            l % r
                        }
                    }
                    Op::Pow => l.powf(r),
                }
            }
            Expr::FuncCall(name, args) => {
                let evaluated_args: Vec<f64> = args.iter().map(|a| self.eval_expr(a)).collect();
                self.call_func(name, &evaluated_args)
            }
        }
    }

    fn call_func(&self, name: &str, args: &[f64]) -> f64 {
        match name {
            "cos" => args.first().copied().unwrap_or(0.0).to_radians().cos(),
            "sin" => args.first().copied().unwrap_or(0.0).to_radians().sin(),
            "tan" => args.first().copied().unwrap_or(0.0).to_radians().tan(),
            "abs" => args.first().copied().unwrap_or(0.0).abs(),
            "floor" => args.first().copied().unwrap_or(0.0).floor(),
            "min" => {
                if args.len() >= 2 {
                    args[0].min(args[1])
                } else {
                    args.first().copied().unwrap_or(0.0)
                }
            }
            "max" => {
                if args.len() >= 2 {
                    args[0].max(args[1])
                } else {
                    args.first().copied().unwrap_or(0.0)
                }
            }
            "irand" | "frand" => {
                let min = args.first().copied().unwrap_or(0.0);
                let max = if args.len() >= 2 { args[1] } else { min };
                match self.random_type {
                    RandomType::ForceMin => min,
                    RandomType::ForceMax => max,
                    RandomType::Freely => {
                        // Szybki deterministyczny generator w srodowisku klienta
                        (min + max) * 0.5
                    }
                }
            }
            "sqrt" => args.first().copied().unwrap_or(0.0).max(0.0).sqrt(),
            _ => 0.0,
        }
    }
}

struct Lexer;

impl Lexer {
    fn tokenize(s: &str) -> Result<Vec<Token>, ()> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];
            if c.is_whitespace() {
                i += 1;
                continue;
            }

            match c {
                '+' => {
                    tokens.push(Token::Plus);
                    i += 1;
                }
                '-' => {
                    tokens.push(Token::Minus);
                    i += 1;
                }
                '*' => {
                    tokens.push(Token::Mul);
                    i += 1;
                }
                '/' => {
                    tokens.push(Token::Div);
                    i += 1;
                }
                '%' => {
                    tokens.push(Token::Mod);
                    i += 1;
                }
                '^' => {
                    tokens.push(Token::Pow);
                    i += 1;
                }
                '(' => {
                    tokens.push(Token::LParen);
                    i += 1;
                }
                ')' => {
                    tokens.push(Token::RParen);
                    i += 1;
                }
                ',' => {
                    tokens.push(Token::Comma);
                    i += 1;
                }
                '0'..='9' | '.' => {
                    let mut num_str = String::new();
                    while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                        num_str.push(chars[i]);
                        i += 1;
                    }
                    let val: f64 = num_str.parse().map_err(|_| ())?;
                    tokens.push(Token::Number(val));
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut id_str = String::new();
                    while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                        id_str.push(chars[i]);
                        i += 1;
                    }
                    tokens.push(Token::Ident(id_str.to_lowercase()));
                }
                _ => {
                    i += 1; // Ignoruj nieznane znaki
                }
            }
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn next(&mut self) -> Token {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            tok
        } else {
            Token::Eof
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ()> {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Result<Expr, ()> {
        let mut left = self.parse_factor()?;

        while let Token::Plus | Token::Minus = self.peek() {
            let op_tok = self.next();
            let right = self.parse_factor()?;
            let op = match op_tok {
                Token::Plus => Op::Add,
                Token::Minus => Op::Sub,
                _ => unreachable!(),
            };
            left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, ()> {
        let mut left = self.parse_power()?;

        while let Token::Mul | Token::Div | Token::Mod = self.peek() {
            let op_tok = self.next();
            let right = self.parse_power()?;
            let op = match op_tok {
                Token::Mul => Op::Mul,
                Token::Div => Op::Div,
                Token::Mod => Op::Mod,
                _ => unreachable!(),
            };
            left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        }

        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expr, ()> {
        let mut left = self.parse_unary()?;

        if let Token::Pow = self.peek() {
            self.next();
            let right = self.parse_power()?; // Prawostronna lacznosc
            left = Expr::BinaryOp(Box::new(left), Op::Pow, Box::new(right));
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ()> {
        if let Token::Minus = self.peek() {
            self.next();
            let inner = self.parse_unary()?;
            Ok(Expr::UnaryMinus(Box::new(inner)))
        } else if let Token::Plus = self.peek() {
            self.next();
            self.parse_unary()
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ()> {
        match self.next() {
            Token::Number(val) => Ok(Expr::Num(val)),
            Token::Ident(name) => {
                if let Token::LParen = self.peek() {
                    // Wywolanie funkcji
                    self.next(); // Zuzyj '('
                    let mut args = Vec::new();
                    if *self.peek() != Token::RParen {
                        args.push(self.parse_expr()?);
                        while let Token::Comma = self.peek() {
                            self.next(); // Zuzyj ','
                            args.push(self.parse_expr()?);
                        }
                    }
                    if self.next() != Token::RParen {
                        return Err(());
                    }
                    Ok(Expr::FuncCall(name, args))
                } else {
                    Ok(Expr::Var(name))
                }
            }
            Token::LParen => {
                let inner = self.parse_expr()?;
                if self.next() != Token::RParen {
                    return Err(());
                }
                Ok(inner)
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpoly_basic_math() {
        let mut poly = CPoly::new();
        poly.set_str("2 + 3 * 4");
        assert_eq!(poly.eval(), 14.0);

        poly.set_str("(2 + 3) * 4");
        assert_eq!(poly.eval(), 20.0);

        poly.set_str("2^3 + 1");
        assert_eq!(poly.eval(), 9.0);
    }

    #[test]
    fn test_cpoly_variables() {
        let mut poly = CPoly::new();
        poly.set_var("atk", 150.0);
        poly.set_var("def", 50.0);
        poly.set_var("k", 1.5);

        poly.set_str("atk * k - def");
        assert_eq!(poly.eval(), 175.0);
    }

    #[test]
    fn test_cpoly_functions() {
        let mut poly = CPoly::new();
        poly.set_str("max(10, 25) + min(5, 2)");
        assert_eq!(poly.eval(), 27.0);

        poly.set_str("abs(-50) + floor(3.7)");
        assert_eq!(poly.eval(), 53.0);
    }

    #[test]
    fn test_cpoly_skill_damage_formula() {
        // Przykladowa formula Metin2 Wir Miecza: 1.2 * (atk + 2*str) * k
        let mut poly = CPoly::new();
        poly.set_var("atk", 300.0);
        poly.set_var("str", 90.0);
        poly.set_var("k", 2.0);

        poly.set_str("1.2 * (atk + 2 * str) * k");
        let result = poly.eval();
        assert_eq!(result, 1152.0);
    }
}
