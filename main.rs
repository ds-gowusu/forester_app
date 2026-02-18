use dotenvy::dotenv;
use forester::{app::App, db::pool::init_db_pool, domain::inventory::InventoryState, services::inventory::InventoryService, tui::tui_run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

   let  db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in the enviroment or .env file");
    let db_pool = init_db_pool(&db_url).await?;
    let service = InventoryService::new(db_pool);
    let inventory_state = InventoryState::new(service).await?;

    let mut terminal = ratatui::init();
    let mut app = App::new(inventory_state);

    tui_run(&mut terminal, &mut app)?;

    ratatui::restore();

    Ok(())
}
