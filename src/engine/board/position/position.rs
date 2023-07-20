#[derive(Clone, Copy)]
pub enum CastleOptions {
    None,
    Right,
    Left,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub colum: i8,
    pub row: i8,
}

#[derive(Clone, Copy)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub castle: CastleOptions,
}
