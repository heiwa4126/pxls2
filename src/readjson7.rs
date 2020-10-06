use crate::pkg;
use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashSet;

fn main2i686(jsonfile: &str) -> String {
    return jsonfile[..jsonfile.len() - 5].to_string() + "_i686.json";
}

pub fn read_json7(jsonfile: &str) -> Result<Vec<pkg::Pkg>> {
    let s1 = pkg::read_main_json(jsonfile)?;

    let i686 = pkg::read_i686_json(&main2i686(jsonfile))?;
    // let i686: HashSet<_> = i686.iter().cloned().collect();
    let i686: HashSet<_> = i686.iter().collect();

    // println!("{:#?}", s1);
    // println!("{:#?}", i686);

    let mut pkgs: Vec<pkg::Pkg> = Vec::new();

    for l in s1 {
        if let Some(i) = l.desc.find(" from ") {
            if let Some((ver, arch)) = pkg::ver_arch(&l.desc[..i]) {
                let pkg = pkg::Pkg::new(&l.name, ver, arch);
                pkgs.push(pkg);
                if arch == pkg::ARCH_X86 && i686.contains(&l.name) {
                    let pkg = pkg::Pkg::new(&l.name, ver, pkg::ARCH_I686);
                    pkgs.push(pkg);
                }
            } else {
                return Err(anyhow!("Unknown arch. '{}'", l.desc));
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
        let rc = read_json7("./test/7/web02.json").expect("ERROR");
        println!("{:#?}", rc);
    }
    #[test]
    fn test_i686() {
        assert_eq!(main2i686("./test/7/web02.json"), "./test/7/web02_i686.json");
    }
}
