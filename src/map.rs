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
    catagory: FeatureType,
    color: i32,
    glyph: char,
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
    stack: Vec<Positionable>,
    location: Position,
}

pub struct Level {
    depth: i32,
    map: Vec<FloorStack>,
    window: Window,
    annotations: String,
}
