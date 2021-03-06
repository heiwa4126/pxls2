#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
use pxls2::{config, run};
use std::env;

fn main() -> anyhow::Result<()> {
    let cnf = {
        let args: Vec<String> = env::args().collect();
        config::parse(&args)?
    };

    if cnf.yaml_mode {
        // yaml mode
        eprintln!("json_dir={}, yaml_file={}", &cnf.json_dir, &cnf.out_file);
        run::run_yaml(&cnf.json_dir, &cnf.out_file)
    } else {
        // normal mode
        eprintln!("json_dir={}, Excel_file={}", cnf.json_dir, cnf.out_file);
        run::run_excel(&cnf.json_dir, &cnf.out_file)
    }
}
