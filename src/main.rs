extern crate pancurses;
use pancurses::{endwin, initscr};

fn main() {
    let window = initscr();
    window.printw("Hello RustHack");
    window.refresh();
    let subwindow = window.derwin(20, 80, 2, 2).unwrap();
    //let subwindow2 = window.subwin(20, 30, 10, 65).unwrap();

    //window.resize(10, 20);
    subwindow.printw("here is a box");

    subwindow.border('|', '|', '-', '-', '+', '+', '+', '+');
    // subwindow2.border('|', '|', '-', '-', '+', '+', '+', '+');

    window.border('|', '|', '-', '-', '+', '+', '+', '+');

    window.getch();
    endwin();
}
