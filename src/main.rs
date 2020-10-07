extern crate getopts;
use getopts::Options;
use pxls2::run;
use std::env;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

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
    println!("{} v{} (xxxxxxxxx)", PKG_NAME, PKG_VERSION);
    std::process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("y", "", "YAMLモード");
    opts.optflag("h", "help", "ヘルプを表示");
    opts.optflag("v", "version", "バージョンを表示");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(opts);
    } else if matches.opt_present("v") {
        print_version();
    }

    let mut json_dir: &str = "./test/7";
    let mut out_file: &str = "./Book1.xlsx";
    if matches.opt_present("y") {
        out_file = "./updates_db.yaml";
    }

    let argc = matches.free.len();
    if argc >= 1 {
        json_dir = &matches.free[0];
    }
    if argc >= 2 {
        out_file = &matches.free[1];
    }

    let rc = if matches.opt_present("y") {
        // yaml mode
        eprintln!("json_dir={}, yaml_file={}", json_dir, out_file);
        run::run_yaml(json_dir, out_file)
    } else {
        // normal mode
        eprintln!("json_dir={}, Excel_file={}", json_dir, out_file);
        run::run(json_dir, out_file)
    };

    if let Err(e) = rc {
        panic!("{:?}", e);
    }
}
