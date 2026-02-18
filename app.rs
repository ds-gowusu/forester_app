use ratatui::widgets::TableState;

use crate::domain::inventory::InventoryState;

pub struct App {
    pub tab_index: usize,
    pub stands_tble_state: TableState,
    pub trees_tble_state: TableState,
    pub inventory_state: InventoryState,
    
}

impl App {
    pub fn new(state: InventoryState) -> Self {

        Self {
            tab_index: 0,
            stands_tble_state: TableState::default().with_selected(Some(0)),
            trees_tble_state: TableState::default().with_selected(Some(0)),
            inventory_state: state,
            
        }
    }
}
