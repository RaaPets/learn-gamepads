use eyre::Result;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

//  //  //  //  //  //  //  //
pub type RUNNER = fn(
    &mut ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
) -> Result<()>;

pub struct App;

impl App {
    pub fn new() -> Self {
        trace!(" + App::new()");

        Self
    }

    pub fn run(&mut self, runner: RUNNER) -> Result<()> {
        let mut terminal = ratatui::init();
        let result = runner(&mut terminal);
        ratatui::restore();
        if let Err(ref e) = result {
            error!("{}", e);
        }
        result
    }
}

impl Drop for App {
    fn drop(&mut self) {
        trace!(" - App::drop()");
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_true() -> Result<()> {
        let mut app = App::new();

        let status = app.run(|_| {
            println!("inside test_run_true..");
            Ok(())
        });

        status
    }
}
