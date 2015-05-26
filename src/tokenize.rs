#[derive(PartialEq)]
pub enum Operator {
    IncCell,     // +
    DecCell,     // -
    IncPtr,      // >
    DecPtr,      // <
    Print,       // .
    Read,        // ,
    JumpZero,    // [
    Loop,        // ]
}


pub fn tokenize(source: String) -> Vec<Operator> {
    let mut tokens: Vec<Operator> = Vec::new();
    for c in source.chars() {
        let o: Option<Operator> = match c {
            '+' => Some(Operator::IncCell),
            '-' => Some(Operator::DecCell),
            '>' => Some(Operator::IncPtr),
            '<' => Some(Operator::DecPtr),
            '.' => Some(Operator::Print),
            ',' => Some(Operator::Read),
            '[' => Some(Operator::JumpZero),
            ']' => Some(Operator::Loop),
            _ => None
        };
        match o {
            Some(operator) => tokens.push(operator),
            None => {}
        };
    };
    tokens
}
