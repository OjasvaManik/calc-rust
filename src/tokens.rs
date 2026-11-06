#[derive(Debug, PartialEq)]
pub enum Token {
    NumInteger(i64),
    NumFloat(f64),
    Plus,
    Minus,
    Star,
    FSlash,
    Percent,
    LBracket,
    RBracket,
    Exit,
}
