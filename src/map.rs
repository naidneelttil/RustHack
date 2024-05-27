extern crate pancurses;
use pancurses::Window;

enum FeatureType {
    Water,
    Lava,
    Tree,
    Wall,
    Corridor,
    Altar,
}

pub struct DungeonFeature {
    pub catagory: FeatureType,
    pub color: i32,
    pub glyph: char,
}

enum Positionable {
    FeatureType,
    Item,
    Monst,
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

// this is the stack that each tile of the floor is made of
pub struct FloorStack {
    pub stack: Vec<Positionable>,
    pub location: Position,
}

pub struct Level {
    pub depth: i32,
    pub map: Vec<FloorStack>,
    pub window: Window,
    pub annotations: String,
}
