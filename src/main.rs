use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

mod app;
mod map;
mod widget;

use app::*;
use map::*;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let map: Vec<Vec<T>> = vec![
        vec![T::W, T::W, T::W, T::W],
        vec![T::W, T::P, T::F, T::W],
        vec![T::W, T::F, T::F, T::W],
        vec![T::W, T::W, T::W, T::W],
    ];

    let mut app = App {
        exit: false,
        player: [1, 1],
        stringmap: printmap(&map),
        vecmap: map.clone(),
    };

    app.run(&mut terminal)?;

    ratatui::restore();
    Ok(())
}
