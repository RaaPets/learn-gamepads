use eyre::Result;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

//  //  //  //  //  //  //  //
pub type RUNNER = fn(
    &mut App,
    &mut ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
) -> Result<()>;

pub struct App {
    exiting: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let app = App { exiting: false };
        trace!(" + App::new()");
        Ok(app)
    }

    pub fn run(&mut self, runner: RUNNER) -> Result<()> {
        let mut terminal = ratatui::init();
        let result = runner(self, &mut terminal);
        ratatui::restore();
        if let Err(ref e) = result {
            error!("{}", e);
        }
        result
    }

    pub fn set_exiting(&mut self) {
        self.exiting = true;
    }
    pub fn is_exiting(&self) -> bool {
        self.exiting
    }
}

impl Drop for App {
    fn drop(&mut self) {
        if self.exiting {
            trace!(" - App::drop() with TRUE exiting");
        } else {
            trace!(" - App::drop() with FALSE exiting");
        }
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod tests {
    use super::*;
    use eyre::WrapErr;

    #[test]
    fn test_run_true() -> Result<()> {
        let mut app = App::new().wrap_err("invoking App::new() in tests::test_run()")?;

        let status = app.run(|app, _| {
            println!("inside test_run_true..");
            app.set_exiting();
            Ok(())
        });
        assert!(app.is_exiting() == true);

        status
    }

    #[test]
    fn test_run_false() -> Result<()> {
        let mut app = App::new().wrap_err("invoking App::new() in tests::test_run()")?;

        let status = app.run(|_, _| {
            println!("inside test_run_false..");
            Ok(())
        });
        assert!(app.is_exiting() == false);

        status
    }
}
