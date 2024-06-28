use std::{
    collections::VecDeque,
    fmt::{self, Formatter},
    fs,
};

#[derive(Debug)]
pub enum UnaryOp {
    NegInteger,
    Not,
    StringToInt,
    IntToString,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Less,
    More,
    Eq,
    Or,
    And,
    Concat,
    Prefix,
    Drop,
    Apply,
}

pub enum Token {
    Bool(bool),
    Int(i64),
    String(Vec<u8>),
    UnaryOp(UnaryOp, Box<Token>),
    BinaryOp(BinaryOp, Box<Token>, Box<Token>),
    If(Box<Token>, Box<Token>, Box<Token>),
    // TODO: Lambda abstraction
    // TODO: Evaluation
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Bool(b) => write!(f, "Bool({})", b),
            Token::Int(i) => write!(f, "Int({})", i),
            Token::String(s) => {
                let s = String::from_utf8(s.clone()).unwrap();
                write!(f, "String({})", s)
            }
            Token::UnaryOp(op, inner) => write!(f, "UnaryOp({:?}, {:?})", op, inner),
            Token::BinaryOp(op, first, second) => {
                write!(f, "BinaryOp({:?}, {:?}, {:?})", op, first, second)
            }
            Token::If(cond, first, second) => {
                write!(f, "If({:?}, {:?}, {:?})", cond, first, second)
            }
        }
    }
}

const BASE: i64 = 94;
const START: u8 = 33;
const ALPH: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";

pub fn encode_string(s: &str) -> String {
    let mut res = vec![b'S'];
    for c in s.chars() {
        let pos = ALPH.find(c).unwrap();
        res.push(pos as u8 + START);
    }
    String::from_utf8(res).unwrap()
}

pub fn parse(tokens: &mut VecDeque<Vec<u8>>) -> Token {
    let s = tokens.pop_front().unwrap();
    let indicator = s[0];
    match indicator {
        b'T' => Token::Bool(true),
        b'F' => Token::Bool(false),
        b'I' => {
            let mut res = 0;
            for i in 1..s.len() {
                res = res * BASE + (s[i] - START) as i64;
            }
            Token::Int(res)
        }
        b'S' => {
            let mut str = vec![];
            for i in 1..s.len() {
                let pos = s[i] - START;
                str.push(ALPH.as_bytes()[pos as usize]);
            }
            Token::String(str)
        }
        b'U' => {
            let inner = Box::new(parse(tokens));
            match s[1] {
                b'-' => Token::UnaryOp(UnaryOp::NegInteger, inner),
                b'!' => Token::UnaryOp(UnaryOp::Not, inner),
                b'#' => Token::UnaryOp(UnaryOp::StringToInt, inner),
                b'$' => Token::UnaryOp(UnaryOp::IntToString, inner),
                _ => panic!("Invalid unary operator: {}", s[1]),
            }
        }
        b'B' => {
            let first = Box::new(parse(tokens));
            let second = Box::new(parse(tokens));
            match s[1] {
                b'+' => Token::BinaryOp(BinaryOp::Add, first, second),
                b'-' => Token::BinaryOp(BinaryOp::Sub, first, second),
                b'*' => Token::BinaryOp(BinaryOp::Mul, first, second),
                b'/' => Token::BinaryOp(BinaryOp::Div, first, second),
                b'%' => Token::BinaryOp(BinaryOp::Mod, first, second),
                b'<' => Token::BinaryOp(BinaryOp::Less, first, second),
                b'>' => Token::BinaryOp(BinaryOp::More, first, second),
                b'=' => Token::BinaryOp(BinaryOp::Eq, first, second),
                b'|' => Token::BinaryOp(BinaryOp::Or, first, second),
                b'&' => Token::BinaryOp(BinaryOp::And, first, second),
                b'.' => Token::BinaryOp(BinaryOp::Concat, first, second),
                b'T' => Token::BinaryOp(BinaryOp::Prefix, first, second),
                b'D' => Token::BinaryOp(BinaryOp::Drop, first, second),
                b'$' => Token::BinaryOp(BinaryOp::Apply, first, second),
                _ => panic!("Invalid binary operator: {}", s[1]),
            }
        }
        b'?' => {
            let cond = Box::new(parse(tokens));
            let first = Box::new(parse(tokens));
            let second = Box::new(parse(tokens));
            Token::If(cond, first, second)
        }
        b'L' => {
            panic!("Lambda abstraction not implemented yet")
        }
        b'v' => {
            panic!("Evaluation not implemented yet")
        }
        _ => panic!("Invalid token: {}", indicator),
    }
}

pub fn parse_string(input: &str) -> Token {
    let mut words: VecDeque<Vec<u8>> = input
        .split_whitespace()
        .map(|s| s.as_bytes().to_vec())
        .collect();
    let res = parse(&mut words);
    assert!(words.is_empty());
    res
}

#[test]
fn test() {
    let input = fs::read_to_string("inputs/start.txt").unwrap();
    eprintln!("Input: {:?}", input);
    let res = parse_string(&input);
    eprintln!("Res: {:?}", res);
}

#[test]
fn simple() {
    let input = "S'%4}).$%8";
    let res = parse_string(input);
    eprintln!("Res: {:?}", res);
}

#[test]
fn encode_str_test() {
    let input = "get index";
    let res = encode_string(input);
    eprintln!("Res: {:?}", res);
}
