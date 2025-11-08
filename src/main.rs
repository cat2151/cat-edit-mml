use anyhow::Result;
use cat_edit_mml::app::App;

fn main() -> Result<()> {
    let mut app = App::new()?;
    app.run()
}


