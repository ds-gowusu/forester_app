
use ratatui::{
    Frame, 
    layout::{Constraint, Layout, Rect}, 
    style::{Modifier, Style}, 
    text::Line, 
    widgets::{Block, Row, Table, TableState, Tabs},
};

use crate::app::App;


pub fn render_inventory_screen(f: &mut Frame<'_>, area: Rect, app: &mut App) {
    let vertical = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0)
    ]);

    let [tab, content] = vertical.areas(area);

 

    render_tabs(f, tab, app);

    match app.tab_index {
        0 => render_trees_table(f, content, app),
        1 => render_stands_table(f, content, app),
        _ => {}
    }

}

pub fn render_trees_table(f: &mut Frame<'_>, area: Rect, app: &mut App) {
  let rows = app.inventory_state.trees.iter()
        .map(|s| {
            let dbh = s.dbh.map(|v| v.to_string()).unwrap_or("-".to_string());
            let height = s.height.map(|v| v.to_string()).unwrap_or("-".to_string());
            let tree_id = s.tree_id.map(|v| v.to_string()).unwrap_or("-".to_string());

            Row::new(vec![
                s.plantation.clone(),
                s.area_type.clone(),
                s.plot_id.clone(),
                tree_id,
                dbh,
                height,
                s.measurement_year.to_string()
            ])
        });

    let table = Table::new(
        rows,
        [
          Constraint::Length(20),
          Constraint::Length(20),
          Constraint::Length(10),
          Constraint::Length(10),
          Constraint::Length(10),
          Constraint::Length(10),
          Constraint::Length(20)
        ]
    )
        .header(Row::new(
            vec![
                "Plantation",
                "Area Type",
                "Plot ID",
                "Tree ID",
                "DBH (cm)",
                "Height (m)",
                "Measurement Year"
            ]
        )).block(Block::bordered().title("Blocks"))
           .highlight_symbol(">> ");

    f.render_stateful_widget(table, area, &mut app.trees_tble_state);

}


pub fn render_stands_table(f: &mut Frame<'_>, area: Rect, app: &mut App) {

   let rows = app.inventory_state.stands.iter()
        .map(|s| {
            Row::new(vec![
                s.plantation.clone(),
                s.block_name.clone(),
                s.reserve.clone(),
                s.year_planted.to_string(),
                s.block_area.to_string(),
                s.productive.map(|v| v.to_string())
                       .unwrap_or_else(|| "None".to_string())
                
            ])
        });

    let table = Table::new(
        rows,
        [
          Constraint::Length(20),
          Constraint::Length(20),
          Constraint::Length(20),
          Constraint::Length(10),
          Constraint::Length(10),
          Constraint::Length(10)
        ]
    )
        .header(Row::new(
            vec![
                "Plantation",
                "Block Name",
                "Reserve",
                "Year Planted",
                "Block Area",
                "Productive Area"
            ]
        )).block(Block::bordered().title("Blocks"))
           .highlight_symbol(">> ");

    f.render_stateful_widget(table, area, &mut app.stands_tble_state);
        
}

pub fn render_tabs(f: &mut Frame, area: Rect, app: &App) {
    let tabs_headers: Vec<_> = ["Block Register", "Tree Data"]
        .iter()
        .map(|t| Line::from(*t))
        .collect();

    let tabs = Tabs::new(tabs_headers)
        .select(app.tab_index)
        .block(Block::bordered().title("Inventory"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(tabs, area);
}



pub fn next_tab(app: &mut App) {
    app.tab_index = (app.tab_index + 1) % 2;
}

pub fn prev_tab(app: &mut App) {
    if app.tab_index == 0 {
        app.tab_index = 1;
    } else {
        app.tab_index -= 1;
    }
}

pub fn next_row(tb_state: &mut TableState, len: usize) {
    if let Some(i) = tb_state.selected() {
        let next = if i >= len - 1 {0} else {
            i + 1
        };

        tb_state.select(Some(next));
    }
}




