use crossterm::ExecutableCommand;
use crossterm::event::{self, KeyEvent};
use crossterm::style::Stylize;
use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{self, disable_raw_mode, enable_raw_mode},
    Result,
};
use std::fmt::Error;
use std::io::{stdout, Write};
use std::io::Stdout;
use std::time::Duration;

enum KeyBinding {
    None,
    Clear,
    Quit,
    Square,
    Triangle,
}

fn process_input_events() -> Option<KeyBinding> {
    if let Event::Key(event) = event::read().expect("Reading key event failed") {
        match event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => Some(KeyBinding::Quit),
	    KeyEvent {
		code: KeyCode::Char('s'),
		..
	    } => Some(KeyBinding::Square),
	    KeyEvent {
		code: KeyCode::Char('t'),
		..
	    } => Some(KeyBinding::Triangle),
	    KeyEvent {
		code: KeyCode::Char('c'),
		..
	    } => Some(KeyBinding::Clear),
            _ => Some(KeyBinding::None),
        }
    } else {
        return None;
    }
}

fn print_square(mut stdout: &Stdout, x: u16, y: u16) -> Result<()> {
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let x_0 = x / 4;
    let x_1 = x_0 * 3;
    let y_0 = y / 4;
    let y_1 = y_0 * 3;

    for j in x_0..x_1 {
	for k in y_0..y_1 {
	    move_to_position(&stdout, j, y_0)?;
	    print_point(&stdout)?;
	    move_to_position(&stdout, j, y_1)?;
	    print_point(&stdout)?;
	    move_to_position(&stdout, x_0, k)?;
	    print_point(&stdout)?;
	    move_to_position(&stdout, x_1, k)?;
	    print_point(&stdout)?;
	}
    }
    
    Ok(())
}

fn print_triangle(mut stdout: &Stdout, x: u16, y: u16) -> Result<()> {
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    // let x_0 = x / 3;
    // let x_1 = x_0 * 2;
    // let y_0 = y / 3;
    // let y_1 = y_0 * 2;

    // for j in x_0..x_1 {
    // 	for k in y_0..y_1 {
    // 	    move_to_position(&stdout, j, y_1)?;
    // 	    print_point(&stdout)?;
    // 	    move_to_position(&stdout, x_0, k)?;
    // 	    print_point(&stdout)?;
    // 	}
    // }

    let mut array = vec![vec![0; x.into()]; y.into()];

    array[10][10] = 1;
    array[11][10] = 1;
    array[12][10] = 1;
    array[13][10] = 1;
    array[14][10] = 1;
    array[11][11] = 1;
    array[13][11] = 1;
    array[12][12] = 1;

    for (i, row) in array.iter().enumerate() {
	for (j, _col) in row.iter().enumerate() {
	    if array[i][j] == 1 {
		move_to_position(&stdout, i as u16, j as u16)?;
		print_point(&stdout)?;
	    }
	}
    }
    
    Ok(())
}

fn move_to_position(mut stdout: &Stdout, x: u16, y: u16) -> Result<()> {
    stdout.execute(crossterm::cursor::MoveToRow(y))?;
    stdout.execute(crossterm::cursor::MoveToColumn(x))?;
    Ok(())
}

fn print_point(mut stdout: &Stdout) ->  Result<()> {
    stdout.execute(crossterm::style::Print("*".blue()))?;
    Ok(())
}

fn main() -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let (x, y) = crossterm::terminal::size()?;
    
    enable_raw_mode()?;

    
    loop {
        match process_input_events() {
            Some(KeyBinding::Quit) => {
		stdout.execute(terminal::Clear(terminal::ClearType::All))?;
		break;
	    },
	    Some(KeyBinding::Clear) => {
		stdout.execute(terminal::Clear(terminal::ClearType::All))?;
	    },
	    Some(KeyBinding::Square) => {
		print_square(&mut stdout, x, y)?;
	    },
	    Some(KeyBinding::Triangle) => {
		print_triangle(&mut stdout, x, y)?;
	    },
            Some(KeyBinding::None) => {},
            None => {}
        }
    }

    Ok(())
}
