use crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, ui::inventory::{next_row, next_tab, prev_tab}};

pub fn handle_events(key: KeyEvent, state: &mut App) -> anyhow::Result<()> {
    match key.code {
        KeyCode::Tab => next_tab(state),
        KeyCode::Left => prev_tab(state),
        KeyCode::Down => {
            if state.tab_index == 0 {
                next_row(&mut state.stands_tble_state, 200);
            } else {
                next_row(&mut state.trees_tble_state, 200);
            }
        },
        _ => {}
    }
    Ok(())
} 
