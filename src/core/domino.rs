
/// Represents a domino 
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Domino(pub Landscape, pub Landscape, pub u8);


/// Represents the possible landscapes of a domino
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Landscape {
    Farm = 0,
    Field = 1,
    Forest = 2,
    Mine = 3,
    Lake = 4,
    Swamp = 5,
    Castle,
    None,
}


/// Represents all dominoes in the game
pub const DOMINOES: [Domino; 48] = [
    Domino(Landscape::Farm, Landscape::Farm, 0),
    Domino(Landscape::Farm, Landscape::Farm, 0),
    Domino(Landscape::Forest, Landscape::Forest, 0),
    Domino(Landscape::Forest, Landscape::Forest, 0),
    Domino(Landscape::Forest, Landscape::Forest, 0),
    Domino(Landscape::Forest, Landscape::Forest, 0),
    Domino(Landscape::Lake, Landscape::Lake, 0),
    Domino(Landscape::Lake, Landscape::Lake, 0),
    Domino(Landscape::Lake, Landscape::Lake, 0),
    Domino(Landscape::Field, Landscape::Field, 0),
    Domino(Landscape::Field, Landscape::Field, 0),
    Domino(Landscape::Swamp, Landscape::Swamp, 0),
    Domino(Landscape::Farm, Landscape::Forest, 0),
    Domino(Landscape::Farm, Landscape::Lake, 0),
    Domino(Landscape::Farm, Landscape::Field, 0),
    Domino(Landscape::Farm, Landscape::Swamp, 0),
    Domino(Landscape::Forest, Landscape::Lake, 0),
    Domino(Landscape::Forest, Landscape::Field, 0),
    Domino(Landscape::Farm, Landscape::Forest, 1),
    Domino(Landscape::Farm, Landscape::Lake, 1),
    Domino(Landscape::Farm, Landscape::Field, 1),
    Domino(Landscape::Farm, Landscape::Swamp, 1),
    Domino(Landscape::Farm, Landscape::Mine, 1),
    Domino(Landscape::Forest, Landscape::Farm, 1), 
    Domino(Landscape::Forest, Landscape::Farm, 1), 
    Domino(Landscape::Forest, Landscape::Farm, 1), 
    Domino(Landscape::Forest, Landscape::Farm, 1), 
    Domino(Landscape::Forest, Landscape::Lake, 1), 
    Domino(Landscape::Forest, Landscape::Field, 1), 
    Domino(Landscape::Lake, Landscape::Farm, 1), 
    Domino(Landscape::Lake, Landscape::Farm, 1), 
    Domino(Landscape::Lake, Landscape::Forest, 1), 
    Domino(Landscape::Lake, Landscape::Forest, 1), 
    Domino(Landscape::Lake, Landscape::Forest, 1), 
    Domino(Landscape::Lake, Landscape::Forest, 1), 
    Domino(Landscape::Field, Landscape::Farm, 1), 
    Domino(Landscape::Field, Landscape::Lake, 1), 
    Domino(Landscape::Swamp, Landscape::Farm, 1), 
    Domino(Landscape::Swamp, Landscape::Field, 1), 
    Domino(Landscape::Mine, Landscape::Farm, 1), 
    Domino(Landscape::Field, Landscape::Farm, 2), 
    Domino(Landscape::Field, Landscape::Lake, 2), 
    Domino(Landscape::Swamp, Landscape::Farm, 2), 
    Domino(Landscape::Swamp, Landscape::Field, 2), 
    Domino(Landscape::Mine, Landscape::Farm, 2), 
    Domino(Landscape::Mine, Landscape::Swamp, 2), 
    Domino(Landscape::Mine, Landscape::Swamp, 2), 
    Domino(Landscape::Mine, Landscape::Farm, 3), 
];

