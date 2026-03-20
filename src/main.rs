use anyhow::Result;
use cat_edit_mml::app::App;
use cat_edit_mml::self_update::run_self_update;

fn main() -> Result<()> {
    if std::env::args().skip(1).next().as_deref() == Some("update") {
        let should_exit = run_self_update()?;
        if should_exit {
            std::process::exit(0);
        }
        return Ok(());
    }

    let mut app = App::new()?;
    app.run()
}
