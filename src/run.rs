use crate::{excel1, ls, readjson7};
use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

pub fn run(jsondir: &str, excelfile: &str) -> Result<()> {
    let hosts = ls::ls(jsondir)?;
    if hosts.len() == 0 {
        return Err(anyhow!("No JSON files at '{}'", jsondir));
    }
    let mut e1 = excel1::Excel1::new(excelfile);
    for host in hosts {
        let pkgs = readjson7::read(&ls::host2file(&host, jsondir))?;
        e1.add_host(&host, pkgs)?;
    }
    e1.finish()?;

    Ok(())
}

pub fn run_yaml(jsondir: &str, yaml_file: &str) -> Result<()> {
    let hosts = ls::ls(jsondir)?;
    if hosts.len() == 0 {
        return Err(anyhow!("No JSON files at '{}'", jsondir));
    }
    let mut map = BTreeMap::new();

    for host in hosts {
        let pkgs = readjson7::read(&ls::host2file(&host, jsondir))?;
        map.insert(host, pkgs);
    }

    let mut f = File::create(yaml_file)?;
    serde_yaml::to_writer(&f, &map)?;
    f.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test7_excel() {
        if let Err(e) = run("test/7", "tmp/7.xlsx") {
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
