pub mod inventory;

use ratatui::Frame;

use crate::{app::App, ui::inventory::render_inventory_screen};

pub fn view_ui(f: &mut Frame<'_>, app: &mut App) {
   let area = f.area();
    render_inventory_screen(f, area, app);
}
