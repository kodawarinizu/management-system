use sistema_gestion::infrastructure::cli::menu::Menu;

#[tokio::main]
async fn main() {
    let menu = Menu::new();
    if let Err(e) = menu.main().await {
        eprintln!("{}", e);
    }
}
