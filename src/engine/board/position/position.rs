#[derive(Clone, Copy)]
pub enum CastleOptions {
    None,
    KingSide,
    QueenSide,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub colum: i8,
    pub row: i8,
}

#[derive(Clone, Copy)]
pub struct LegalMove {
    pub from: Position,
    pub to: Position,
    pub castle: CastleOptions,
}
