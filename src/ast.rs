use std::fmt::Display;

use crate::eval::EvalError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exp {
    Nil,
    Bool(bool),
    Integer(i64),
    String(String),
    Symbol(String),
    Lambda(String, Box<Exp>),
    Apply(Box<Exp>, Box<Exp>),
    List(Vec<Exp>),
    If(Box<Exp>, Box<Exp>, Box<Exp>),
    Quote(Box<Exp>),
    Let((String, Box<Exp>), Box<Exp>),
    Case(Box<Exp>, Vec<(Exp, Exp)>),
    BuildIn(fn(&[Exp]) -> Result<Exp, EvalError>, Vec<Exp>),
}

impl Display for Exp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exp::Nil => write!(f, "nil"),
            Exp::Bool(bool) => write!(f, "{}", bool.to_string()),
            Exp::Integer(integer) => write!(f, "{}", integer.to_string()),
            Exp::String(str) => write!(f, "{}", str),
            Exp::Symbol(sym) => write!(f, "{}", sym),
            Exp::Lambda(arg, exp) => write!(f, "(\\ ({}) {})", arg, exp),
            Exp::Apply(exp1, exp2) => write!(f, "({} {})", exp1, exp2),
            Exp::List(exps) => write!(
                f,
                "({})",
                exps.iter()
                    .map(|exp| exp.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Exp::If(cond, then, else_) => write!(f, "(if {} {} {})", cond, then, else_),
            Exp::Quote(exp) => write!(f, "'{}", exp),
            Exp::Let((bind, exp1), exp2) => write!(f, "(let ({} {}) {})", bind, exp1, exp2),
            Exp::Case(cond, matchers) => write!(
                f,
                "(case {} {})",
                cond,
                matchers
                    .iter()
                    .map(|(match_, exp)| format!("({} {})", match_, exp))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Exp::BuildIn(_, args) => write!(
                f,
                "(#buildin {})",
                args.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
        }
    }
}

pub fn nil() -> Exp {
    Exp::Nil
}

pub fn bool(b: bool) -> Exp {
    Exp::Bool(b)
}

pub fn integer(i: i64) -> Exp {
    Exp::Integer(i)
}

pub fn string(s: &str) -> Exp {
    Exp::String(s.to_string())
}

pub fn symbol(sym: &str) -> Exp {
    Exp::Symbol(sym.to_string())
}

pub fn lambda(param: &str, body: Exp) -> Exp {
    Exp::Lambda(param.to_string(), Box::new(body))
}

pub fn apply(e1: Exp, e2: Exp) -> Exp {
    Exp::Apply(Box::new(e1), Box::new(e2))
}

pub fn list(list: &[Exp]) -> Exp {
    Exp::List(list.to_vec())
}

pub fn if_(cond: Exp, then: Exp, else_: Exp) -> Exp {
    Exp::If(Box::new(cond), Box::new(then), Box::new(else_))
}

pub fn let_(bind: (&str, Exp), exp: Exp) -> Exp {
    Exp::Let((bind.0.to_string(), Box::new(bind.1)), Box::new(exp))
}

pub fn case(exp: Exp, cases: &[(Exp, Exp)]) -> Exp {
    Exp::Case(Box::new(exp), cases.to_vec())
}

pub fn quote(e: Exp) -> Exp {
    Exp::Quote(Box::new(e))
}

pub fn buildin(f: fn(&[Exp]) -> Result<Exp, EvalError>, args: &[Exp]) -> Exp {
    Exp::BuildIn(f, args.to_vec())
}
