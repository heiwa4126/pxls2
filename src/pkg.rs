use crate::arch::Arch;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::cmp::Ord;
use std::fmt;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, Ord, Eq, PartialOrd, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pkg {
    pub name: String,
    pub version: String,
    pub arch: Arch,
}

impl fmt::Display for Pkg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_s())
    }
}

// impl Ord for Pkg {
//     // #[derive(Ord)]で何が生成されるか自信がない...
//     fn cmp(&self, other: &Self) -> Ordering {
//         let rc = self.name.cmp(&other.name);
//         if rc != Ordering::Equal {
//             return rc;
//         }
//         let rc = self.version.cmp(&other.version);
//         if rc != Ordering::Equal {
//             return rc;
//         }
//         self.arch.cmp(&other.arch)
//     }
// }

impl Pkg {
    pub fn to_s(&self) -> String {
        format!("{}-{}.{}", self.name, self.version, self.arch)
    }

    pub fn new(
        name: impl Into<String>,
        ver: impl Into<String>,
        arch: impl Into<String>,
    ) -> Result<Pkg> {
        Ok(Pkg::new0(name, ver, FromStr::from_str(&arch.into())?))
    }
    pub fn new0(name: impl Into<String>, ver: impl Into<String>, arch: Arch) -> Pkg {
        Pkg {
            name: name.into(),
            version: ver.into(),
            arch: arch,
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
    // println!("{}", jsonfile);
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

    #[test]
    fn test_pkg_cmp() {
        use std::cmp::Ordering;
        let p1 = Pkg::new("x", "1.0", ARCH_X86).expect("ERROR!");
        let p2 = Pkg::new("x", "1.0", ARCH_I686).expect("ERROR!");
        let p3 = Pkg::new("x", "1.0", ARCH_X86).expect("ERROR!");
        assert_eq!(p1.cmp(&p2), Ordering::Less);
        assert_eq!(p2.cmp(&p1), Ordering::Greater);
        assert_eq!(p1.cmp(&p3), Ordering::Equal);
    }
}
