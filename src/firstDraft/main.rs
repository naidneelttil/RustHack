extern crate pancurses;
mod items;
mod map;
mod monst;
use arr_macro::arr;
use items::Item;
use map::*;
use monst::Monst;
use pancurses::{endwin, initscr, newwin, Input::Character, Window};

struct GameState {
    dungeon: Vec<map::Level>,
}

fn main() {
    // initialized the current game state, making the dungeon
    let mut current_state = GameState {
        dungeon: Vec::new(),
    };

    // initialize the window and and make the 80x20 game window
    // and make the window in which the player's stats is shown
    let window = initscr();
    pancurses::noecho();
    window.refresh();
    let subwindow = newwin(20, 80, 5, 2);
    let player_window = newwin(5, 80, 25, 2);
    let message_window = newwin(3, 80, 1, 2);

    subwindow.refresh();
    player_window.refresh();
    message_window.refresh();
    //let subwindow = pancurses::derwin(20, 80, 5, 2).unwrap;
    //let player_window = window.derwin(5, 80, 25, 2).unwrap();
    //let message_window = window.derwin(3, 80, 1, 2).unwrap();

    // specify the boarders for each of the player windows
    //window.border('|', '|', '-', '-', '+', '+', '+', '+');
    subwindow.border('|', '|', '-', '-', '-', '-', '-', '-');
    player_window.border('|', '|', '-', '-', '+', '+', '+', '+');
    message_window.border('|', '|', '-', '-', '+', '+', '+', '+');

    //subwindow.mvaddstr(1, 1, "content");
    //window.draw_box('|', '-');
    //subwindow.draw_box('|', '-');
    //player_window.draw_box('|', '-');
    //message_window.draw_box('|', '-');

    subwindow.refresh();
    player_window.refresh();
    message_window.refresh();

    message_window.mvprintw(1, 1, "Welcome to RustHack!");
    message_window.refresh();
    // construct the first level of the game with dots on every square
    let mut first_level = Level {
        depth: 0,
        annotations: String::new(),
        map: [['.'; 80]; 20],
        map_obj: arr![arr![Vec::new(); 80]; 20],
        objects: Vec::new(),
    };

    // make the player with the correct glyph and window
    let mut player = Monst {
        health: 10,
        power: 1,
        ac: 10,
        inventory: Vec::new(),
        glyph: '@',
        location: Position { x: 2, y: 3 },
    };

    // TODO: this should also be a function that takes objects and stores it in the current level
    // put the player glyph on the first_level map
    first_level.map[player.location.x as usize][player.location.y as usize] = player.glyph;

    // put the first level in the dungeon game state
    current_state.dungeon.push(first_level);

    // TODO: this is supposed to be a function that takes the current level and prints it to the
    // screen,player's glyph. change this eventually.
    populate_board(&subwindow, current_state.dungeon.get(0).unwrap());
    subwindow.mvaddch(player.location.x, player.location.y, player.glyph);
    player_window.refresh();
    // TODO: this has to be a loop of getting the player's input and altering the game accordingly
    let input = window.getch().unwrap();

    match input {
        // remove player from former location
        // level.map[player.location.x as usize]
        Character('h') => player.location.x -= 1,
        Character('l') => player.location.x += 1,
        Character('j') => player.location.y -= 1,
        Character('k') => player.location.x += 1,
        _ => (),
    }

    endwin();
    //debugging
    println!(
        "this is the struct representation of the first level {:#?}",
        current_state.dungeon.get(0)
    );
    println!("this is the input {:#?}", input);
}

fn populate_all_floor(subwindow: &Window, level: &mut Level) {
    for i in 1..19 {
        for j in 1..79 {
            let floor = DungeonFeature {
                catagory: map::FeatureType::RoomFloor,
                color: 0,
                glyph: '.',
                location: Position { x: i, y: j },
            };

            level.map_obj[i as usize][j as usize].push(Positionable::Feature(floor));
        }
    }
    subwindow.refresh();
}
fn populate_board(subwindow: &Window, level: &Level) {
    for i in 1..19 {
        for j in 1..79 {
            subwindow.mvaddch(i, j, level.map[i as usize][j as usize]);
        }
    }
    subwindow.refresh();
}
