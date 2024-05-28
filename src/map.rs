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
pub struct FloorStack {
    pub stack: Vec<Positionable>,
    //pub location: Position,
}

pub struct Level {
    pub depth: i32,
    pub map: [[FloorStack; 80]; 20],
    pub window: Window,
    pub annotations: String,
}

impl Level {
    fn populate_map(&mut self) {
        for i in 0..80 {
            for j in 0..20 {
                self.map[i][j] = FloorStack { stack: Vec::new() };
            }
        }
    }
}
