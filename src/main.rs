extern crate getopts;
use anyhow::Result;
use getopts::Options;
use pxls2::run;
use std::env;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_REVISION: Option<&'static str> = option_env!("PKG_REVISION");

fn print_usage(opts: Options) {
    let brief = format!(
        r#"Usage:
 {} <JSON files directory> <output Excel file(xlsx)>
 {} -y <JSON files directory> <output update db file (YAML)>
 {} [options]"#,
        PKG_NAME, PKG_NAME, PKG_NAME
    );
    println!("{}", opts.usage(&brief));
    std::process::exit(2);
}

fn print_version() {
    println!(
        "{} v{} ({})",
        PKG_NAME,
        PKG_VERSION,
        PKG_REVISION.unwrap_or_else(|| "unknown")
    );
    std::process::exit(2);
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("y", "", "YAMLモード");
    opts.optflag("h", "help", "ヘルプを表示");
    opts.optflag("v", "version", "バージョンを表示");

    let matches = opts.parse(&args[1..])?;

    // help & version
    if matches.opt_present("h") {
        print_usage(opts);
    } else if matches.opt_present("v") {
        print_version();
    }

    // set default value
    let mut json_dir = "./test/7";
    let mut out_file = if matches.opt_present("y") {
        "./updates_db.yaml"
    } else {
        "./Book1.xlsx"
    };

    let argc = matches.free.len();
    if argc >= 1 {
        json_dir = &matches.free[0];
    }
    if argc >= 2 {
        out_file = &matches.free[1];
    }

    if matches.opt_present("y") {
        // yaml mode
        eprintln!("json_dir={}, yaml_file={}", json_dir, out_file);
        run::run_yaml(json_dir, out_file)
    } else {
        // normal mode
        eprintln!("json_dir={}, Excel_file={}", json_dir, out_file);
        run::run(json_dir, out_file)
    }
}
