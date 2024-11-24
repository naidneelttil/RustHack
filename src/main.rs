


use std::io::{Result, Write, stdout};
use crossterm::{QueueableCommand, execute, cursor, terminal};
use crossterm::event::{read, Event};


fn main() -> Result<()> {


    init();
    let mut stdout = stdout();
    stdout.queue(terminal::Clear(terminal::ClearType::All));
    stdout.queue(terminal::Clear(terminal::ClearType::Purge));

    stdout.queue(cursor::MoveTo(5,5));
    

     loop {

      match read()? {
         Event::FocusGained => println!("FocusGained"),
         Event::FocusLost => println!("FocusLost"),
         Event::Key(event) => println!("{:?}", event),
         Event::Mouse(event) => println!("{:?}", event),
         Event::Paste(data) => println!("{:?}", data),
         Event::Resize(width, height) => println!("New size {}x{}", width, height),
      }

  }

  //stdout.flush();

  restore();
}


/// Initialize the terminal
fn init() -> Result<()> {
    execute!(stdout(), terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    Ok(())
}

/// Restore the terminal to its original state
fn restore() -> Result<()> {
    execute!(stdout(), terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
