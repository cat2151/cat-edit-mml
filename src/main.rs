use anyhow::Result;
use cat_edit_mml::app::App;
use cat_edit_mml::cli::{parse_args, Command};
use cat_edit_mml::self_update::{run_check, run_update};

fn main() -> Result<()> {
    let args = parse_args();

    match args.command {
        Some(Command::Check) => run_check(),
        Some(Command::Update) => run_update(),
        None => {
            let mut app = App::new()?;
            app.run()
        }
    }
}
