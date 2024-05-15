
extern crate pancurses;
use pancurses::{initscr, endwin};



fn main() {
    let window = initscr();
    window.printw("Hello RustHack");
    window.refresh();
    window.getch();
    endwin();
}
