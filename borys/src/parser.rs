use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{self, Formatter},
    fs,
    rc::Rc,
};

// https://www.minjiezha.com/tech/2011/01/19/A-Simple-Lambda-Calculus-Evaluator-III.html
// https://laurenar.net/posts/lambda_calculus_interpreter/

#[derive(Debug, Clone)]
pub enum UnaryOp {
    NegInteger,
    Not,
    StringToInt,
    IntToString,
}

#[derive(Debug, Clone)]
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
}

#[derive(Clone)]
pub enum Token {
    Bool(bool),
    Int(i64),
    String(Vec<u8>),
    UnaryOp(UnaryOp, Rc<Token>),
    BinaryOp(BinaryOp, Rc<Token>, Rc<Token>),
    If(Rc<Token>, Rc<Token>, Rc<Token>),
    Application(Rc<Token>, Rc<Token>),
    Abstraction(usize, Rc<Token>),
    Id(usize),
}

impl Token {
    pub fn create_var(&self) -> (usize, Rc<Token>) {
        match self {
            Token::Abstraction(i, inner) => (*i, inner.clone()),
            _ => panic!("Expected CreateVar, got {:?}", self),
        }
    }

    pub fn int(&self) -> i64 {
        match self {
            Token::Int(i) => *i,
            _ => panic!("Expected Int, got {:?}", self),
        }
    }

    pub fn bool(&self) -> bool {
        match self {
            Token::Bool(b) => *b,
            _ => panic!("Expected Bool, got {:?}", self),
        }
    }

    pub fn string(&self) -> Vec<u8> {
        match self {
            Token::String(s) => s.clone(),
            _ => panic!("Expected String, got {:?}", self),
        }
    }
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
            Token::Abstraction(i, inner) => write!(f, "CreateVar({}, {:?})", i, inner),
            Token::Application(lhs, rhs) => write!(f, "Application({:?}, {:?})", lhs, rhs),
            Token::Id(i) => write!(f, "UseVar({})", i),
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

fn parse_integer(s: &[u8]) -> i64 {
    let mut res = 0;
    for &c in s {
        res = res * BASE + (c - START) as i64;
    }
    res
}

pub fn parse(tokens: &mut VecDeque<Vec<u8>>) -> Token {
    let s = tokens.pop_front().unwrap();
    let indicator = s[0];
    match indicator {
        b'T' => Token::Bool(true),
        b'F' => Token::Bool(false),
        b'I' => Token::Int(parse_integer(&s[1..])),
        b'S' => {
            let mut str = vec![];
            for i in 1..s.len() {
                let pos = s[i] - START;
                str.push(ALPH.as_bytes()[pos as usize]);
            }
            Token::String(str)
        }
        b'U' => {
            let inner = Rc::new(parse(tokens));
            match s[1] {
                b'-' => Token::UnaryOp(UnaryOp::NegInteger, inner),
                b'!' => Token::UnaryOp(UnaryOp::Not, inner),
                b'#' => Token::UnaryOp(UnaryOp::StringToInt, inner),
                b'$' => Token::UnaryOp(UnaryOp::IntToString, inner),
                _ => panic!("Invalid unary operator: {}", s[1]),
            }
        }
        b'B' => {
            let first = Rc::new(parse(tokens));
            let second = Rc::new(parse(tokens));
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
                b'$' => Token::Application(first, second),
                _ => panic!("Invalid binary operator: {}", s[1]),
            }
        }
        b'?' => {
            let cond = Rc::new(parse(tokens));
            let first = Rc::new(parse(tokens));
            let second = Rc::new(parse(tokens));
            Token::If(cond, first, second)
        }
        b'L' => {
            let i = parse_integer(&s[1..]);
            let inner = Rc::new(parse(tokens));
            Token::Abstraction(i as usize, inner)
        }
        b'v' => {
            let i = parse_integer(&s[1..]);
            Token::Id(i as usize)
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
    assert!(words.is_empty(), "left: {:?}", words);
    res
}

pub fn eval(token: &Token) -> Token {
    match token {
        Token::Bool(_) | Token::Int(_) | Token::String(_) | Token::Id(_) => token.clone(),
        Token::UnaryOp(op, inner) => {
            let inner = eval(inner);
            match op {
                UnaryOp::NegInteger => Token::Int(-inner.int()),
                UnaryOp::Not => Token::Bool(!inner.bool()),
                UnaryOp::StringToInt => {
                    let mut res = 0;
                    for c in inner.string() {
                        let pos = ALPH.find(c as char).unwrap();
                        res = res * BASE + pos as i64;
                    }
                    Token::Int(res)
                }
                UnaryOp::IntToString => {
                    let mut res = vec![];
                    let mut n = inner.int();
                    while n > 0 {
                        res.push(ALPH.as_bytes()[(n % BASE) as usize]);
                        n /= BASE;
                    }
                    res.reverse();
                    Token::String(res)
                }
            }
        }
        Token::BinaryOp(op, first, second) => {
            let first = eval(first);
            let second = eval(second);
            match op {
                BinaryOp::Add => Token::Int(first.int() + second.int()),
                BinaryOp::Sub => Token::Int(first.int() - second.int()),
                BinaryOp::Mul => Token::Int(first.int() * second.int()),
                BinaryOp::Div => Token::Int(first.int() / second.int()),
                BinaryOp::Mod => Token::Int(first.int() % second.int()),
                BinaryOp::Less => Token::Bool(first.int() < second.int()),
                BinaryOp::More => Token::Bool(first.int() > second.int()),
                BinaryOp::Eq => match (&first, &second) {
                    (Token::Int(a), Token::Int(b)) => Token::Bool(a == b),
                    (Token::Bool(a), Token::Bool(b)) => Token::Bool(a == b),
                    (Token::String(a), Token::String(b)) => Token::Bool(a == b),
                    _ => panic!("Invalid equality check: {:?} == {:?}", first, second),
                },
                BinaryOp::Or => Token::Bool(first.bool() || second.bool()),
                BinaryOp::And => Token::Bool(first.bool() && second.bool()),
                BinaryOp::Concat => {
                    let mut res = first.string();
                    res.extend(second.string());
                    Token::String(res)
                }
                BinaryOp::Prefix => {
                    let res = second.string()[..first.int() as usize].to_vec();
                    Token::String(res)
                }
                BinaryOp::Drop => {
                    let res = second.string()[first.int() as usize..].to_vec();
                    Token::String(res)
                }
            }
        }
        Token::If(cond, first, second) => {
            let cond = eval(cond);
            if cond.bool() {
                eval(first)
            } else {
                eval(second)
            }
        }
        Token::Abstraction(i, inner) => {
            let inner = eval(inner);
            Token::Abstraction(*i, Rc::new(inner))
        }
        Token::Application(e1, e2) => match e1.as_ref() {
            Token::Abstraction(i, inner) => {
                let inner = substitute(inner, *i, e2.clone());
                eval(&inner)
            }
            _ => {
                let e1 = eval(e1);
                let e2 = eval(e2);
                eval(&Token::Application(Rc::new(e1), Rc::new(e2)))
            }
        },
    }
}

fn substitute(token: &Token, var: usize, with: Rc<Token>) -> Rc<Token> {
    match token {
        Token::Bool(_) | Token::Int(_) | Token::String(_) => Rc::new(token.clone()),
        Token::UnaryOp(op, inner) => {
            Rc::new(Token::UnaryOp(op.clone(), substitute(inner, var, with)))
        }
        Token::BinaryOp(op, first, second) => Rc::new(Token::BinaryOp(
            op.clone(),
            substitute(first, var, with.clone()),
            substitute(second, var, with),
        )),
        Token::If(cond, first, second) => Rc::new(Token::If(
            substitute(cond, var, with.clone()),
            substitute(first, var, with.clone()),
            substitute(second, var, with),
        )),
        Token::Abstraction(i, inner) => {
            if *i == var {
                // shadows, don't substitute
                Rc::new(Token::Abstraction(*i, inner.clone()))
            } else {
                Rc::new(Token::Abstraction(*i, substitute(inner, var, with)))
            }
        }
        Token::Id(i) => {
            if *i == var {
                with
            } else {
                Rc::new(Token::Id(*i))
            }
        }
        Token::Application(lhs, rhs) => Rc::new(Token::Application(
            substitute(lhs, var, with.clone()),
            substitute(rhs, var, with),
        )),
    }
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

#[test]
fn parse_simple_lambda() {
    let input = "B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK";
    let res = parse_string(input);
    eprintln!("Res: {:?}", res);
    let eval_res = eval(&res);
    eprintln!("Eval res: {:?}", eval_res);
    assert_eq!(eval_res.string(), b"Hello World!");
}

#[test]
fn parse_smaller() {
    let input = "B$ B$ L# L! v# I& I$";
    let res = parse_string(input);
    eprintln!("Res: {:?}", res);
    let eval_res = eval(&res);
    eprintln!("Eval res: {:?}", eval_res);
    assert_eq!(eval_res.int(), 5);
}

#[test]
fn int_to_string() {
    let input = "U$ I4%34";
    let res = parse_string(input);
    eprintln!("Res: {:?}", res);
    let eval_res = eval(&res);
    eprintln!("Eval res: {:?}", eval_res);
    assert_eq!(eval_res.string(), b"test");
}

#[test]
fn string_to_int() {
    let input = "U# S4%34";
    let res = parse_string(input);
    eprintln!("Res: {:?}", res);
    let eval_res = eval(&res);
    eprintln!("Eval res: {:?}", eval_res);
    assert_eq!(eval_res.int(), 15818151);
}
