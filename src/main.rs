use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

mod elm_arch;

//  //  //  //  //  //  //  //
fn main() -> Result<()> {
    log_init();

    let args:Vec<String> = std::env::args().collect();

    let mut terminal = ratatui::init();
    let result = match args[1].as_str() {
        "elm" => {
            elm_arch::App::new().run(&mut terminal)
        },
        _ => {
            todo!("there is no default (yet)")
        },
    };
    //let result = App::new().run(&mut terminal);

    ratatui::restore();
    if let Err(ref e) = result {
        error!("{}", e);
    }

    trace!("############\n<-----\n.\n ");
    result
}

//  //  //  //  //  //  //  //
fn log_init() {
    raalog::init()
        .expect("unable init log system")
        .set_file_mode("/tmp/rust_debug.log")
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
