use crate::{
    ast,
    interpreter::{
        builtin::Builtin,
        environment::Environment,
    },
};
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub struct Integer {
    pub value: i128,
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub struct Boolean {
    pub value: bool,
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
pub struct MString {
    pub value: String,
}

impl fmt::Display for MString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct MArray {
    pub elements: Vec<MObject>,
}

impl fmt::Display for MArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.elements.iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ReturnValue {
    pub value: Box<MObject>,
}

impl fmt::Display for ReturnValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
pub struct MError {
    pub value: String,
}

impl fmt::Display for MError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR: {}", self.value)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Function {
    pub params: Vec<ast::Identifier>,
    pub body: ast::BlockStatement,
    pub env: Environment,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fn(")?;
        write!(
            f,
            "{}",
            self.params.iter()
                .map(|p| format!("{}", p))
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        write!(f, ") {}", self.body)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MObject {
    Int(Integer),
    Bool(Boolean),
    Str(MString),
    Array(MArray),
    Return(ReturnValue),
    Err(MError),
    Fn(Function),
    Builtin(Builtin),
    Null,
}

impl fmt::Display for MObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MObject::Int(x) => write!(f, "{}", x),
            MObject::Bool(x) => write!(f, "{}", x),
            MObject::Str(x) => write!(f, "{}", x),
            MObject::Array(x) => write!(f, "{}", x),
            MObject::Return(x) => write!(f, "{}", x),
            MObject::Err(x) => write!(f, "{}", x),
            MObject::Fn(x) => write!(f, "{}", x),
            MObject::Builtin(x) => write!(f, "{}", x),
            MObject::Null => write!(f, "null"),
        }
    }
}
