use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
#[serde(tag = "type", content = "op")]
pub enum Op {
    Plus,
    Minus,
    Star,
    Slash,
    Modulus,
    PlusPlus,
    MinusMinus,

    And,
    Pipe,
    Carat,
    Tilde,
    BitShiftLeft,
    BitShiftRight,
    UnsignedBitShiftRight,

    FatArrow,
    ThinArrow,
    ExclamationPoint,
    QuestionMark,
    Colon,

    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    ModulusEqual,
    AndEqual,
    PipeEqual,
    CaratEqual,
    TildeEqual,
    ShiftLeftEqual,
    ShiftRightEqual,
    UnsignedShiftRightEqual,

    AndAnd,
    PipePipe,
    EqualEqual,
    NotEqual,
    GreaterThan,
    GreaterThanEqualTo,
    LessThan,
    LessThanEqualTo,

    Spaceship,

    Other(String),
}

impl From<Op> for String {
    fn from(operation: Op) -> Self {
        match operation {
            Op::Plus => "+".into(),
            Op::Minus => "-".into(),
            Op::Star => "*".into(),
            Op::Slash => "/".into(),
            Op::Modulus => "%".into(),
            Op::PlusPlus => "++".into(),
            Op::MinusMinus => "--".into(),

            Op::And => "&".into(),
            Op::Pipe => "|".into(),
            Op::Carat => "^".into(),
            Op::Tilde => "~".into(),
            Op::BitShiftLeft => "<<".into(),
            Op::BitShiftRight => ">>".into(),
            Op::UnsignedBitShiftRight => ">>>".into(),

            Op::FatArrow => "=>".into(),
            Op::ThinArrow => "->".into(),
            Op::ExclamationPoint => "!".into(),
            Op::QuestionMark => "?".into(),
            Op::Colon => ":".into(),

            Op::Equal => "=".into(),
            Op::PlusEqual => "+=".into(),
            Op::MinusEqual => "-=".into(),
            Op::StarEqual => "*=".into(),
            Op::SlashEqual => "/=".into(),
            Op::ModulusEqual => "%=".into(),
            Op::AndEqual => "&=".into(),
            Op::PipeEqual => "|=".into(),
            Op::CaratEqual => "^=".into(),
            Op::TildeEqual => "~=".into(),
            Op::ShiftLeftEqual => "<<=".into(),
            Op::ShiftRightEqual => ">>=".into(),
            Op::UnsignedShiftRightEqual => ">>>=".into(),

            Op::AndAnd => "&&".into(),
            Op::PipePipe => "||".into(),
            Op::EqualEqual => "==".into(),
            Op::NotEqual => "!=".into(),
            Op::GreaterThan => ">".into(),
            Op::GreaterThanEqualTo => ">=".into(),
            Op::LessThan => "<".into(),
            Op::LessThanEqualTo => "<=".into(),

            Op::Spaceship => "<=>".into(),

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
            "%" => Op::Modulus,
            "++" => Op::PlusPlus,
            "--" => Op::MinusMinus,

            "&" => Op::And,
            "|" => Op::Pipe,
            "^" => Op::Carat,
            "~" => Op::Tilde,
            "<<" => Op::BitShiftLeft,
            ">>" => Op::BitShiftRight,
            ">>>" => Op::UnsignedBitShiftRight,

            "=>" => Op::FatArrow,
            "->" => Op::ThinArrow,
            "!" => Op::ExclamationPoint,
            "?" => Op::QuestionMark,
            ":" => Op::Colon,

            "=" => Op::Equal,
            "+=" => Op::PlusEqual,
            "-=" => Op::MinusEqual,
            "*=" => Op::StarEqual,
            "/=" => Op::SlashEqual,
            "%=" => Op::ModulusEqual,
            "&=" => Op::AndEqual,
            "|=" => Op::PipeEqual,
            "^=" => Op::CaratEqual,
            "~=" => Op::TildeEqual,
            "<<=" => Op::ShiftLeftEqual,
            ">>=" => Op::ShiftRightEqual,
            ">>>=" => Op::UnsignedShiftRightEqual,

            "&&" => Op::AndAnd,
            "||" => Op::PipePipe,
            "==" => Op::EqualEqual,
            "!=" => Op::NotEqual,
            ">" => Op::GreaterThan,
            ">=" => Op::GreaterThanEqualTo,
            "<" => Op::LessThan,
            "<=" => Op::LessThanEqualTo,

            "<=>" => Op::Spaceship,

            other => Op::Other(String::from(other)),
        }
    }
}
