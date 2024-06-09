#[derive(Debug)]
pub enum ItemType {
    Comensibles,
    Potion,
    Scroll,
    Scrollbook,
    Ring,
    Tool,
    Money,
    Weapon,
}

#[derive(Debug)]
pub enum Beatitude {
    Blessed,
    Cursed,
    Uncursed,
}

#[derive(Debug)]
pub struct Item {
    catagory: ItemType,
    color: i32,
    glyph: char,
    beautide: Beatitude,
}
