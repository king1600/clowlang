use std::fmt;

pub type Context<'a> = (&'a str, &'a str);

pub type SourceLoc = (usize, usize, usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Expr<'a>(ExprType<'a>, SourceLoc);

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a>(TokenType<'a>, SourceLoc);

#[derive(Clone, PartialEq)]
pub struct ParseError<'a>(Error, Context<'a>, SourceLoc);

#[derive(Clone, PartialEq)]
pub enum Error {
    UnterminatedString,
}

#[derive(Clone, PartialEq)]
pub enum Keyword {
    Fun,
    Pub,
    Impl,
    Enum,
    Const,
    Class,
    Do,
    For,
    If,
    Elif,
    Else,
    Match,
    While,
}

#[derive(Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Set,
    Equ,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    Not,
    Xor,
    Shl,
    Shr,
    BitOr,
    BitAnd,
    BitNot,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type<'a> {
    Byte, //u8
    Int,  //i32
    Long, //i64
    Float, //f32
    Double, //f64
    Class(&'a str),
    Generic(&'a str, Vec<Type<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType<'a> {
    Dot,
    Semi,
    Colon,
    Comma,
    Arrow,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LCurly,
    RCurly,
    Int(u64),
    Float(f64),
    Id(&'a str),
    Kw(Keyword),
    Str(&'a str),
    Op(Operator, bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprType<'a> {
    EInt(u64),
    EFloat(f64),
    EId(&'a str),
    EString(&'a str),
    EArray(Vec<Expr<'a>>),
    EUnop(Operator, Box<Expr<'a>>),
    EBinop(Operator, Box<(Expr<'a>, Expr<'a>)>),
    EVar(Type<'a>, Vec<(&'a str, Option<Expr<'a>>)>),
    EFunc(&'a str, i32, Vec<Type<'a>>, Vec<Expr<'a>>),
    EClass(&'a str, i32, Vec<Type<'a>>, Vec<Expr<'a>>),
    EIf(Vec<(Expr<'a>, Vec<Expr<'a>>)>, Option<Vec<Expr<'a>>>),
}

impl<'a> From<&'a str> for Type<'a> {
    fn from(type_name: &'a str) -> Type<'a> {
        match type_name {
            "byte" => Type::Byte,
            "int"  => Type::Int,
            "long" => Type::Long,
            "float" => Type::Float,
            "double" => Type::Double,
            _ => Type::Class(type_name),
        }
    }
}

impl<'a> fmt::Debug for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        let ParseError(error, context, source_loc) = self;
        let (line, column, line_start) = source_loc;
        let (context, source) = context;

        write!(f, "Error on {}:{}:{}> ", context, line, column)
            .and_then(|_| match error {
                UnterminatedString => write!(f, "Untermiated string literal"),
            })
            .and_then(|_| write!(f, "\n  {}", unsafe {
                source
                    .get_unchecked(*line_start..)
                    .lines().next().unwrap_or("EOF")
            }))
    }
}

impl fmt::Debug for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Keyword::*;

        write!(f, "{}", match self {
            Fun => "fn",
            Pub => "pub",
            Impl => "impl",
            Enum => "enum",
            Const => "const",
            Class => "class",
            Do => "do",
            For => "for",
            If => "if",
            Elif => "elif",
            Else => "else",
            Match => "match",
            While => "while",
        })
    }
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Operator::*;

        write!(f, "{}", match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",
            Set => "=",
            Equ => "==",
            Neq => "!=",
            Lt => "<",
            Lte => "<=",
            Gt => ">",
            Gte => ">=",
            And => "&&",
            Or => "||",
            Not => "!",
            Xor => "^",
            Shl => "<<",
            Shr => ">>",
            BitOr => "|",
            BitAnd => "&",
            BitNot => "~",
        })
    }
}