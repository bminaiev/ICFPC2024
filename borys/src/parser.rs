use std::{
    collections::{HashMap, VecDeque},
    fmt::{self, Formatter},
    fs,
    rc::Rc,
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
    UnaryOp(UnaryOp, Rc<Token>),
    BinaryOp(BinaryOp, Rc<Token>, Rc<Token>),
    If(Rc<Token>, Rc<Token>, Rc<Token>),
    CreateVar(usize, Rc<Token>),
    UseVar(usize),
}

impl Token {
    pub fn create_var(&self) -> (usize, Rc<Token>) {
        match self {
            Token::CreateVar(i, inner) => (*i, inner.clone()),
            _ => panic!("Expected CreateVar, got {:?}", self),
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
            Token::CreateVar(i, inner) => write!(f, "CreateVar({}, {:?})", i, inner),
            Token::UseVar(i) => write!(f, "UseVar({})", i),
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
                b'$' => Token::BinaryOp(BinaryOp::Apply, first, second),
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
            Token::CreateVar(i as usize, inner)
        }
        b'v' => {
            let i = parse_integer(&s[1..]);
            Token::UseVar(i as usize)
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

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    Int(i64),
    String(Vec<u8>),
    Lambda(usize, Rc<Token>),
}

impl Value {
    pub fn int(&self) -> i64 {
        match self {
            Value::Int(i) => *i,
            _ => panic!("Expected int, got {:?}", self),
        }
    }

    pub fn bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            _ => panic!("Expected bool, got {:?}", self),
        }
    }

    pub fn string(&self) -> Vec<u8> {
        match self {
            Value::String(s) => s.clone(),
            _ => panic!("Expected string, got {:?}", self),
        }
    }

    pub fn lambda(&self) -> (usize, Rc<Token>) {
        match self {
            Value::Lambda(i, inner) => (*i, inner.clone()),
            _ => panic!("Expected lambda, got {:?}", self),
        }
    }

    pub fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            _ => panic!("Expected same types, got {:?} and {:?}", self, other),
        }
    }
}

fn eval_rec(token: &Token, context: &HashMap<usize, Rc<Token>>) -> Value {
    match token {
        Token::Bool(b) => Value::Bool(*b),
        Token::Int(i) => Value::Int(*i),
        Token::String(s) => Value::String(s.clone()),
        Token::UnaryOp(op, inner) => {
            let inner = eval_rec(inner, context);
            match op {
                UnaryOp::NegInteger => Value::Int(-inner.int()),
                UnaryOp::Not => Value::Bool(!inner.bool()),
                UnaryOp::StringToInt => todo!("StringToInt"),
                UnaryOp::IntToString => todo!("IntToString"),
            }
        }
        Token::BinaryOp(op, first, second) => {
            let first_ev = || eval_rec(first, context);
            let second_ev = || eval_rec(second, context);
            match op {
                BinaryOp::Add => Value::Int(first_ev().int() + second_ev().int()),
                BinaryOp::Sub => Value::Int(first_ev().int() - second_ev().int()),
                BinaryOp::Mul => Value::Int(first_ev().int() * second_ev().int()),
                BinaryOp::Div => Value::Int(first_ev().int() / second_ev().int()),
                BinaryOp::Mod => Value::Int(first_ev().int() % second_ev().int()),
                BinaryOp::Less => Value::Bool(first_ev().int() < second_ev().int()),
                BinaryOp::More => Value::Bool(first_ev().int() > second_ev().int()),
                BinaryOp::Eq => Value::Bool(first_ev().eq(&second_ev())),
                BinaryOp::Or => Value::Bool(first_ev().bool() || second_ev().bool()),
                BinaryOp::And => Value::Bool(first_ev().bool() && second_ev().bool()),
                BinaryOp::Concat => {
                    let mut res = first_ev().string();
                    res.extend(second_ev().string());
                    Value::String(res)
                }
                BinaryOp::Prefix => {
                    let res = second_ev().string()[..first_ev().int() as usize].to_vec();
                    Value::String(res)
                }
                BinaryOp::Drop => {
                    let res = second_ev().string()[first_ev().int() as usize..].to_vec();
                    Value::String(res)
                }
                BinaryOp::Apply => {
                    let mut new_context = context.clone();
                    let (i, inner) = first_ev().lambda();
                    new_context.insert(i, second.clone());
                    eval_rec(&inner, &new_context)
                }
            }
        }
        Token::If(cond, first, second) => {
            let cond = eval_rec(cond, context);
            if cond.bool() {
                eval_rec(first, context)
            } else {
                eval_rec(second, context)
            }
        }
        Token::CreateVar(i, inner) => Value::Lambda(*i, inner.clone()),
        Token::UseVar(i) => {
            let inner = context.get(i).unwrap();
            eval_rec(inner, context)
        }
    }
}

pub fn eval(token: &Token) -> Value {
    let context = HashMap::new();
    eval_rec(token, &context)
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
}
