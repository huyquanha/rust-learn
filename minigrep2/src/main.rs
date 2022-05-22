use std::env;
use std::process;

use minigrep2::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // We don't use unwrap_or_else but use if-let here
    // because the return value is the unit value (),
    // so there's no need to unwrap it. We could still
    // use unwrap_or_else if we like though, just doesn't
    // make a lot of sense.
    if let Err(e) = minigrep2::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
