extern crate pancurses;
mod items;
mod map;
mod monst;
use items::Item;
use map::*;
use monst::Monst;
use pancurses::{endwin, initscr};

struct GameState {
    dungeon: Vec<map::Level>,
}

fn main() {
    let mut current_state = GameState {
        dungeon: Vec::new(),
    };

    let window = initscr();
    window.printw("Hello RustHack");
    //   window.refresh();
    let subwindow = window.derwin(20, 80, 2, 2).unwrap();

    // construct the first level of the game

    let mut first_level = Level {
        depth: 0,
        annotations: String::new(),
        map: [['.'; 80]; 20],
        objects: Vec::new(),
    };

    let player = Monst {
        health: 10,
        power: 1,
        ac: 10,
        inventory: Vec::new(),
        glyph: '@',
        location: Position { x: 2, y: 3 },
    };

    first_level.map[player.location.x as usize][player.location.y as usize] = player.glyph;

    //let subwindow2 = window.subwin(20, 30, 10, 65).unwrap();
    current_state.dungeon.push(first_level);

    //window.resize(10, 20);
    let player_window = window.derwin(5, 80, 23, 2).unwrap();

    subwindow.border('|', '|', '-', '-', '-', '-', '-', '-');
    player_window.border('|', '|', '-', '-', '+', '+', '+', '+');

    subwindow.mvaddch(player.location.x, player.location.y, player.glyph);
    window.border('|', '|', '-', '-', '+', '+', '+', '+');

    window.getch();

    //TODO see if you can add a glyph player that can walk around the screen
    endwin();
    println!(
        "this is the struct representation of the first level {:#?}",
        current_state.dungeon.get(0)
    );
}
