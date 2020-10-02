use anyhow::Result;
use serde::Deserialize;
use std::cmp::Ordering;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pkg {
    name: String,
    version: String,
    arch: String,
}

impl Pkg {
    pub fn cmp(&self, b: &Pkg) -> Ordering {
        let rc = self.name.cmp(&b.name);
        if rc != Ordering::Equal {
            return rc;
        }
        let rc = self.version.cmp(&b.version);
        if rc != Ordering::Equal {
            return rc;
        }
        return self.arch.cmp(&b.arch);
    }
}

pub fn build_pkg(name: &str, ver: &str, arch: &str) -> Pkg {
    return Pkg {
        name: name.to_string(),
        version: ver.to_string(),
        arch: arch.to_string(),
    };
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainPkg {
    pub name: String,
    pub desc: String,
}

pub const ARCH_X86: &str = "x86_64";
pub const ARCH_I686: &str = "i686";
pub const ARCH_NOARCH: &str = "noarch";

pub fn ver_arch(verarch: &str) -> Option<(&str, &str)> {
    for arch in &[ARCH_X86, ARCH_I686, ARCH_NOARCH] {
        if verarch.ends_with(&(".".to_string() + &arch.to_string())) {
            let ver = &verarch[..(verarch.len() - arch.len() - 1)];
            return Some((ver, arch));
        }
    }
    return None;
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verarch() {
        let s = "8-2.el6.noarch";
        if let Some((ver, arch)) = ver_arch(s) {
            assert_eq!(ver, "8-2.el6");
            assert_eq!(arch, "noarch");
        } else {
            panic!("ERROR")
        }
    }
}
