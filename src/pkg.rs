use crate::arch::Arch;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::cmp::Ord;
use std::fmt;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Ord, Eq, PartialOrd, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pkg {
    pub name: String,
    pub version: String,
    pub arch: Arch,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainPkg {
    pub name: String,
    pub desc: String,
}

impl fmt::Display for Pkg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_s())
    }
}

impl Pkg {
    pub fn to_s(&self) -> String {
        format!("{}-{}.{}", self.name, self.version, self.arch)
    }

    // associates

    pub fn new(name: impl Into<String>, ver: impl Into<String>, arch: Arch) -> Pkg {
        Pkg {
            name: name.into(),
            version: ver.into(),
            arch: arch,
        }
    }
    pub fn ver_arch(verarch: &str) -> Result<(&str, Arch)> {
        let arch = Arch::from_ends(verarch)?;
        let ver = &verarch[..(verarch.len() - arch.to_s().len()) - 1];
        Ok((ver, arch.clone()))
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
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        wants: (String, Arch),
    }
    impl TestCase {
        pub fn new(input: impl Into<String>, wants1: impl Into<String>, wants2: Arch) -> TestCase {
            TestCase {
                input: input.into(),
                wants: (wants1.into(), wants2),
            }
        }
    }

    #[test]
    fn test_verarch() {
        for tc in [
            TestCase::new("8-2.el6.noarch", "8-2.el6", Arch::NOARCH),
            TestCase::new("1:5.3.6.1-24.el7.x86_64", "1:5.3.6.1-24.el7", Arch::X86_64),
        ]
        .iter()
        {
            if let Ok((ver, arch)) = Pkg::ver_arch(&tc.input) {
                assert_eq!(ver, tc.wants.0);
                assert_eq!(arch, tc.wants.1);
            } else {
                panic!("ERROR");
            }
        }

        let t9 = TestCase::new("1:5.3.6.1-24.el7.ppc", "1:5.3.6.1-24.el7", Arch::X86_64);
        if let Ok(_) = Pkg::ver_arch(&t9.input) {
            panic!("ERROR");
        }
    }

    #[test]
    fn test_pkg_cmp() {
        use std::cmp::Ordering;
        let p1 = Pkg::new("x", "1.0", Arch::X86_64);
        let p2 = Pkg::new("x", "1.0", Arch::I686);
        let p3 = Pkg::new("x", "1.0", Arch::X86_64);
        assert_eq!(p1.cmp(&p2), Ordering::Less);
        assert_eq!(p2.cmp(&p1), Ordering::Greater);
        assert_eq!(p1.cmp(&p3), Ordering::Equal);
    }
}
