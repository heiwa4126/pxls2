use anyhow::Result;
use pxls2::{config, run};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let cnf = config::parse_config(&args)?;

    if cnf.yaml_mode {
        // yaml mode
        eprintln!("json_dir={}, yaml_file={}", &cnf.json_dir, &cnf.out_file);
        run::run_yaml(&cnf.json_dir, &cnf.out_file)
    } else {
        // normal mode
        eprintln!("json_dir={}, Excel_file={}", cnf.json_dir, cnf.out_file);
        run::run(&cnf.json_dir, &cnf.out_file)
    }
}
