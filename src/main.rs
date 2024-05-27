extern crate pancurses;
mod items;
mod map;
use items::Item;
use map::*;
use pancurses::{endwin, initscr};

struct Monst {
    health: i32,
    power: i32,
    ac: i32,
    inventory: Vec<Item>,
    glyph: char,
    location: Position,
}

struct GameState {
    dungeon: Vec<map::Level>,
}

fn main() {
    let currentState = GameState {
        dungeon: Vec::new(),
    };

    let window = initscr();
    window.printw("Hello RustHack");
    window.refresh();
    let subwindow = window.derwin(20, 80, 2, 2).unwrap();

    // construct the first level of the game
    let first_level = Level {
        depth: 0,
        window: subwindow,
        annotations: String::new(),
        map: Vec::new(),
    };
    //let subwindow2 = window.subwin(20, 30, 10, 65).unwrap();

    let player = Monst {
        health: 10,
        power: 1,
        ac: 10,
        inventory: Vec::new(),
        glyph: '@',
        location: Position { x: 2, y: 3 },
    };

    //window.resize(10, 20);
    let player_window = window.derwin(5, 80, 23, 2).unwrap();
    first_level
        .window
        .border('|', '|', '-', '-', '-', '-', '-', '-');
    player_window.border('|', '|', '-', '-', '+', '+', '+', '+');
    first_level
        .window
        .mvaddch(player.location.x, player.location.y, player.glyph);
    window.border('|', '|', '-', '-', '+', '+', '+', '+');

    window.getch();

    //TODO see if you can add a glyph player that can walk around the screen
    endwin();
}
