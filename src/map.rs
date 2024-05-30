extern crate pancurses;
use crate::items::Item;
use crate::monst::Monst;

#[derive(Debug)]
enum FeatureType {
    Water,
    Lava,
    Tree,
    Wall,
    Corridor,
    Altar,
}

#[derive(Debug)]
pub struct DungeonFeature {
    pub catagory: FeatureType,
    pub color: i32,
    pub glyph: char,
    pub location: Position,
}

#[derive(Debug)]
pub enum Positionable {
    Feature(DungeonFeature),
    Object(Item),
    Monster(Monst),
    None,
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Level {
    pub depth: i32,
    pub map: [[char; 80]; 20],
    pub annotations: String,
    pub objects: Vec<Positionable>,
}
