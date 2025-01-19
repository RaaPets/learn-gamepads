use eyre::{Result};

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

mod runner;
use app_raa_tui::App;

static LOG_FILE: &str = "rust_debug.log";

//  //  //  //  //  //  //  //
fn main() -> Result<()> {
    log_init();

    let mut app = App::new();
    let status = app.run(runner::execute);
    drop(app);

    match status {
        Ok(()) => {
            trace!("############\n<-----\n.\n ");
        }
        Err(ref e) => {
            error!("############\nERROR!\n{}\n<-----\n.\n ", e.to_string());
        }
    };

    status
}

//  //  //  //  //  //  //  //
fn log_init() {
    raalog::init()
        .expect("unable init log system")
        .set_file_mode(&LOG_FILE)
        .expect("unable to set file mode of logger")
        .set_level(raalog::LevelFilter::Trace);

    trace!("\n.\n----->\n############");
    set_panic_hook();
}

//  //  //  //  //  //  //  //
fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        error!("############\nFATAL!\n{}\n<-----\n.\n ", info);
        hook(info);
    }));
}
