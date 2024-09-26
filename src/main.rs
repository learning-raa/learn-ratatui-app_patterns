use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

mod elm_arch;

//  //  //  //  //  //  //  //
fn main() -> Result<()> {
    log_init();

    let args: Vec<String> = std::env::args().collect();

    let mut terminal = ratatui::init();
    let selector = args
        .get(1)
        .ok_or(anyhow::anyhow!("must be an argumeng in command line"))
        .expect("");
    let result = match selector.as_str() {
        "elm" => elm_arch::run(&mut terminal),
        _ => Err(anyhow::anyhow!("unsupported command line argument")),
    };

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
