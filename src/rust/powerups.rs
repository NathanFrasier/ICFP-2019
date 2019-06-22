#[derive(Clone, PartialEq)]
pub enum PowerUp {
    Extension, //{code: 'B'},
    Boost,     // {code: 'F'},
    Drill,     // {code: 'L'},
}

pub trait ByCode {
    fn by_code(code: char) -> Self;
}

impl ByCode for PowerUp {
    fn by_code(code: char) -> Self {
        match code {
            'B' => PowerUp::Extension,
            'F' => PowerUp::Boost,
            'L' => PowerUp::Drill,
            _ => panic!("Unknown powerup code"),
        }
    }
}