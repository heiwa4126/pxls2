use crate::{arch::Arch, pkg::Pkg};
use anyhow::Result;
use std::collections::HashSet;

fn main2i686(jsonfile: &str) -> String {
    jsonfile[..jsonfile.len() - 5].to_string() + "_i686.json"
}

pub fn read(jsonfile: &str) -> Result<Vec<Pkg>> {
    let s1 = Pkg::read_main_json(jsonfile)?;

    let i686 = Pkg::read_i686_json(&main2i686(jsonfile))?;
    let i686: HashSet<_> = i686.iter().collect();

    let mut pkgs: Vec<Pkg> = Vec::new();

    for l in s1 {
        if let Some(i) = l.desc.find(" from ") {
            let (ver, arch) = Pkg::ver_arch(&l.desc[..i])?;
            pkgs.push(Pkg::new(&l.name, ver, arch.clone()));
            if arch == Arch::X86_64 && i686.contains(&l.name) {
                pkgs.push(Pkg::new(&l.name, ver, Arch::I686));
            }
        }
    }
    pkgs.sort_by(|a, b| a.cmp(b));
    return Ok(pkgs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json71() {
        // just read sample data
        let rc = read("./test/7/web02.json").expect("ERROR");
        println!("{:#?}", rc);
    }
    #[test]
    fn test_i686() {
        assert_eq!(main2i686("./test/7/web02.json"), "./test/7/web02_i686.json");
    }
}
