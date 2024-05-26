extern crate pancurses;
use pancurses::{endwin, initscr};

struct Monst {
    health: i32,
    power: i32,
    ac: i32,
    glyph: char,
    location: Position,
}

struct Position {
    x: i32,
    y: i32,
}

fn main() {
    let window = initscr();
    window.printw("Hello RustHack");
    window.refresh();
    let subwindow = window.derwin(20, 80, 2, 2).unwrap();
    //let subwindow2 = window.subwin(20, 30, 10, 65).unwrap();

    let player = Monst {
        health: 10,
        power: 1,
        ac: 10,
        glyph: '@',
        location: Position { x: 2, y: 3 },
    };

    //window.resize(10, 20);
    subwindow.printw("here is a box");

    subwindow.border('|', '|', '-', '-', '+', '+', '+', '+');
    // subwindow2.border('|', '|', '-', '-', '+', '+', '+', '+');

    window.border('|', '|', '-', '-', '+', '+', '+', '+');

    window.getch();

    //TODO see if you can add a glyph player that can walk around the screen
    endwin();
}
