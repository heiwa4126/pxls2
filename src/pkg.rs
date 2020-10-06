use anyhow::Result;
use serde::Deserialize;
use std::cmp::Ordering;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pkg {
    pub name: String,
    pub version: String,
    pub arch: String,
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
        self.arch.cmp(&b.arch)
    }

    // Associated Functions (関連関数)
    pub fn new(name: &str, ver: &str, arch: &str) -> Pkg {
        Pkg {
            name: name.to_string(),
            version: ver.to_string(),
            arch: arch.to_string(),
        }
    }
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
    None
}

pub fn read_main_json(jsonfile: &str) -> Result<Vec<MainPkg>> {
    let json_file_path = Path::new(jsonfile);
    let file = File::open(json_file_path)?;
    let pkgs: Vec<MainPkg> = serde_json::from_reader(file)?;
    Ok(pkgs)
}

pub fn read_i686_json(jsonfile: &str) -> Result<Vec<String>> {
    println!("{}", jsonfile);
    let json_file_path = Path::new(jsonfile);
    let file = File::open(json_file_path)?;
    let pkgs: Vec<String> = serde_json::from_reader(file)?;
    Ok(pkgs)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        wants: (String, String),
    }
    impl TestCase {
        pub fn new(
            input: impl Into<String>,
            wants1: impl Into<String>,
            wants2: impl Into<String>,
        ) -> TestCase {
            TestCase {
                input: input.into(),
                wants: (wants1.into(), wants2.into()),
            }
        }
    }

    #[test]
    fn test_verarch() {
        for tc in [
            TestCase::new("8-2.el6.noarch", "8-2.el6", "noarch"),
            TestCase::new("1:5.3.6.1-24.el7.x86_64", "1:5.3.6.1-24.el7", "x86_64"),
        ]
        .iter()
        {
            if let Some((ver, arch)) = ver_arch(&tc.input) {
                assert_eq!(ver, tc.wants.0);
                assert_eq!(arch, tc.wants.1);
            } else {
                panic!("ERROR");
            }
        }

        let t9 = TestCase::new("1:5.3.6.1-24.el7.ppc", "1:5.3.6.1-24.el7", "ppc");
        if let Some(_) = ver_arch(&t9.input) {
            panic!("ERROR");
        }
    }
}
