use crate::{excel1, ls, pkg, readjson7};
use anyhow::{bail, Result};
use std::collections::BTreeMap;
use std::fs::File;

pub fn run_excel(json_dir: &str, excelfile: &str) -> Result<()> {
    let hosts = ls::ls(json_dir)?;
    if hosts.is_empty() {
        bail!("No JSON files at '{}'", json_dir);
    }
    let mut e1 = excel1::Excel1::new(excelfile);
    for host in hosts {
        let pkgs = readjson7::read(&ls::host2file(&host, json_dir))?;
        e1.add_host(&host, pkgs)?;
    }
    e1.finish()
}

pub fn run_yaml(json_dir: &str, yaml_file: &str) -> Result<()> {
    let hosts = ls::ls(json_dir)?;
    if hosts.is_empty() {
        bail!("No JSON files at '{}'", json_dir);
    }

    let mut map = BTreeMap::new();

    for host in hosts {
        let pkgs = readjson7::read(&ls::host2file(&host, json_dir))?;
        let pkgs: Vec<String> = pkgs.iter().map(pkg::Pkg::to_s).collect();
        map.insert(host, pkgs);
    }

    let f = File::create(yaml_file)?;
    serde_yaml::to_writer(&f, &map)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test7_excel() {
        if let Err(e) = run_excel("test/7", "tmp/7.xlsx") {
            panic!(e);
        }
    }
    #[test]
    fn test_test7_yaml() {
        if let Err(e) = run_yaml("test/7", "tmp/7.yml") {
            panic!(e);
        }
    }
}
