extern crate pancurses;
use crate::items::Item;
use crate::monst::Monst;
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

#[derive(Debug)]
pub enum Positionable {
    FeatureType,
    Item,
    Monst,
    None,
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

// this is the stack that each tile of the floor is made of
#[derive(Debug)]
pub struct FloorStack {
    pub stack: Vec<Positionable>,
    //pub location: Position,
}

#[derive(Debug)]
pub struct Level {
    pub depth: i32,
    pub map: [[FloorStack; 80]; 20],
    pub window: Window,
    pub annotations: String,
}
