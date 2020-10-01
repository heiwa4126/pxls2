use anyhow::Result;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pkg {
    name: String,
    version: String,
    arch: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainPkg {
    name: String,
    desc: String,
}

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct I686Pkg {
//     name: String,
// }

pub fn read_main_json(jsonfile: &str) -> Result<Vec<MainPkg>> {
    let json_file_path = Path::new(jsonfile);
    let file = File::open(json_file_path)?;
    let pkgs: Vec<MainPkg> = serde_json::from_reader(file)?;
    return Ok(pkgs);
}

pub fn read_i686_json(jsonfile: &str) -> Result<Vec<String>> {
    println!("{}", jsonfile);
    let json_file_path = Path::new(jsonfile);
    let file = File::open(json_file_path)?;
    let pkgs: Vec<String> = serde_json::from_reader(file)?;
    return Ok(pkgs);
}
