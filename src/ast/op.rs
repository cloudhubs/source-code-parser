use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum Op {
    Plus,
    Minus,
    Star,
    Slash,
    PlusPlus,
    MinusMinus,

    And,
    Pipe,
    Tilde,

    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    AndEqual,
    PipeEqual,
    TildeEqual,

    AndAnd,
    PipePipe,
    EqualEqual,
    NotEqual,
    GreaterThan,
    GreaterThanEqualTo,
    LessThan,
    LessThanEqualTo,

    Other(String),
}

impl From<Op> for String {
    fn from(operation: Op) -> Self {
        match operation {
            Op::Plus => "+".into(),
            Op::Minus => "-".into(),
            Op::Star => "*".into(),
            Op::Slash => "/".into(),
            Op::PlusPlus => "++".into(),
            Op::MinusMinus => "--".into(),

            Op::And => "&".into(),
            Op::Pipe => "|".into(),
            Op::Tilde => "~".into(),

            Op::Equal => "=".into(),
            Op::PlusEqual => "+=".into(),
            Op::MinusEqual => "-=".into(),
            Op::StarEqual => "*=".into(),
            Op::SlashEqual => "/=".into(),
            Op::AndEqual => "&=".into(),
            Op::PipeEqual => "|=".into(),
            Op::TildeEqual => "~=".into(),

            Op::AndAnd => "&&".into(),
            Op::PipePipe => "||".into(),
            Op::EqualEqual => "==".into(),
            Op::NotEqual => "!=".into(),
            Op::GreaterThan => ">".into(),
            Op::GreaterThanEqualTo => ">=".into(),
            Op::LessThan => "<".into(),
            Op::LessThanEqualTo => "<=".into(),

            Op::Other(value) => value,
        }
    }
}

impl From<&str> for Op {
    fn from(string: &str) -> Op {
        match string {
            "+" => Op::Plus,
            "-" => Op::Minus,
            "*" => Op::Star,
            "/" => Op::Slash,
            "++" => Op::PlusPlus,
            "--" => Op::MinusMinus,

            "&" => Op::And,
            "|" => Op::Pipe,
            "~" => Op::Tilde,

            "=" => Op::Equal,
            "+=" => Op::PlusEqual,
            "-=" => Op::MinusEqual,
            "*=" => Op::StarEqual,
            "/=" => Op::SlashEqual,
            "&=" => Op::AndEqual,
            "|=" => Op::PipeEqual,
            "~=" => Op::TildeEqual,

            "&&" => Op::AndAnd,
            "||" => Op::PipePipe,
            "==" => Op::EqualEqual,
            "!=" => Op::NotEqual,
            ">" => Op::GreaterThan,
            ">=" => Op::GreaterThanEqualTo,
            "<" => Op::LessThan,
            "<=" => Op::LessThanEqualTo,

            other => Op::Other(String::from(other)),
        }
    }
}
