extern crate pancurses;
use pancurses::{endwin, initscr, newwin, Input};

fn main() {
    // initialized the current game state, making the dungeon
    // initialize the window and and make the 80x20 game window
    // and make the window in which the player's stats is shown
    let window = initscr();
    pancurses::noecho();
    window.refresh();
    let mut subwindow = newwin(20, 80, 5, 2);
    let mut player_window = newwin(5, 80, 25, 2);
    let mut message_window = newwin(3, 80, 1, 2);

    let mut msgs: Vec<String> = Vec::new();
    // specify the boarders for each of the player windows
    //window.border('|', '|', '-', '-', '+', '+', '+', '+');
    subwindow.border('|', '|', '-', '-', '+', '+', '+', '+');
    player_window.border('|', '|', '-', '-', '+', '+', '+', '+');
    message_window.border('|', '|', '-', '-', '+', '+', '+', '+');

    subwindow.refresh();
    player_window.refresh();
    message_window.refresh();

    msgs.push(String::from("Welcome to RustHack!"));
    message_window.mvprintw(1, 1, msgs.get(0).unwrap());
    message_window.refresh();

    // handle terminal resize events
    loop {
        let input = window.getch();
        match input {
            Some(Input::KeyResize) => {
                //pancurses::resize_term(0, 0);
                //window.border('|', '|', '-', '-', '+', '+', '+', '+');
                subwindow.erase();
                player_window.erase();
                message_window.erase();

                //let (x, y) = window.get_max_yx();
                subwindow.resize(0, 0);
                player_window.resize(0, 0);
                message_window.resize(0, 0);

                // rewrite all window information here and borders
                message_window.mvprintw(1, 1, msgs.get(0).unwrap());

                player_window.mvprintw(1, 1, "test2!");
                subwindow.mvprintw(1, 1, "test3!");

                subwindow.border('|', '|', '-', '-', '+', '+', '+', '+');
                player_window.border('|', '|', '-', '-', '+', '+', '+', '+');
                message_window.border('|', '|', '-', '-', '+', '+', '+', '+');

                //refresh all the windows so that the changes can be registered
                subwindow.refresh();
                player_window.refresh();
                message_window.refresh();
            }

            Some(Input::Character('q')) => {
                break;
            }

            _ => (),
        }
    }

    // TODO: save the current game state before deleting the window
    //
    // ////////////////////////////////////////////////////////////

    // delete windows and end the game
    subwindow.delwin();
    player_window.delwin();
    message_window.delwin();
    endwin();
}
