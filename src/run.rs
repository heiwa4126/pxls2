use crate::{excel1, ls, readjson7};
use anyhow::{anyhow, Result};

pub fn run(jsondir: &str, excelfile: &str) -> Result<()> {
    let hosts = ls::ls(jsondir)?;
    if hosts.len() == 0 {
        return Err(anyhow!("No JSON files"));
    }
    let mut e1 = excel1::Excel1::new(excelfile);
    for host in hosts {
        let pkgs = readjson7::read(&ls::host2file(&host, jsondir))?;
        e1.add_host(&host, pkgs)?;
    }
    e1.finish()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test7() {
        if let Err(e) = run("test/7", "tmp/7.xlsx") {
            panic!(e);
        }
    }
}
