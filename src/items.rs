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

pub enum Beatitude {
    Blessed,
    Cursed,
    Uncursed,
}

pub struct Item {
    catagory: ItemType,
    color: i32,
    glyph: char,
    beautide: Beatitude,
}
