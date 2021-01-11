pub enum Op {
    Plus,
    Minus,
    Multiple,
    Divide,
    PlusPlus,
    MinusMinus,

    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,

    Equal,
    PlusEqual,
    MinusEqual,
    TimesEqual,
    DivideEqual,
    BitwiseAndEqual,
    BitwiseOrEqual,
    BitwiseNotEqual,

    LogicalAnd,
    LogicalOr,
    LogicalEqual,
    LogicalNotEqual,
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
            Op::Multiple => "*".into(),
            Op::Divide => "/".into(),
            Op::PlusPlus => "++".into(),
            Op::MinusMinus => "--".into(),

            Op::BitwiseAnd => "&".into(),
            Op::BitwiseOr => "|".into(),
            Op::BitwiseNot => "~".into(),

            Op::Equal => "=".into(),
            Op::PlusEqual => "+=".into(),
            Op::MinusEqual => "-=".into(),
            Op::TimesEqual => "*=".into(),
            Op::DivideEqual => "/=".into(),
            Op::BitwiseAndEqual => "&=".into(),
            Op::BitwiseOrEqual => "|=".into(),
            Op::BitwiseNotEqual => "~=".into(),

            Op::LogicalAnd => "&&".into(),
            Op::LogicalOr => "||".into(),
            Op::LogicalEqual => "==".into(),
            Op::LogicalNotEqual => "!=".into(),
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
            "*" => Op::Multiple,
            "/" => Op::Divide,
            "++" => Op::PlusPlus,
            "--" => Op::MinusMinus,

            "&" => Op::BitwiseAnd,
            "|" => Op::BitwiseOr,
            "~" => Op::BitwiseNot,

            "=" => Op::Equal,
            "+=" => Op::PlusEqual,
            "-=" => Op::MinusEqual,
            "*=" => Op::TimesEqual,
            "/=" => Op::DivideEqual,
            "&=" => Op::BitwiseAndEqual,
            "|=" => Op::BitwiseOrEqual,
            "~=" => Op::BitwiseNotEqual,

            "&&" => Op::LogicalAnd,
            "||" => Op::LogicalOr,
            "==" => Op::LogicalEqual,
            "!=" => Op::LogicalNotEqual,
            ">" => Op::GreaterThan,
            ">=" => Op::GreaterThanEqualTo,
            "<" => Op::LessThan,
            "<=" => Op::LessThanEqualTo,

            other => Op::Other(String::from(other)),
        }
    }
}
