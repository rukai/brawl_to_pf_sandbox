use std::env;

use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [list of fighters to export]", program);
    print!("{}", opts.usage(&brief));
}

pub(crate) fn parse_cli() -> Option<CLIResults> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optopt("m", "mod", "name of mod folder in data/ that should be included over brawl", "FOLDER_NAME");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            print_usage(program, opts);
            return None;
        }
    };

    Some(CLIResults {
        mod_name:        matches.opt_str("m"),
        export_fighters: matches.free.iter().map(|x| x.to_lowercase()).collect()
    })
}

pub struct CLIResults {
    pub mod_name:        Option<String>,
    pub export_fighters: Vec<String>,
}
