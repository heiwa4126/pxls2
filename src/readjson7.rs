use crate::pkg;
use anyhow::Result;

fn main2i686(jsonfile: &str) -> String {
    return jsonfile[..jsonfile.len() - 5].to_string() + "_i686.json";
}

pub fn read_json7(jsonfile: &str) -> Result<Vec<pkg::Pkg>> {
    let mut pkgs: Vec<pkg::Pkg> = Vec::new();

    // let s1 = pkg::read_main_json(jsonfile)?;
    // println!("{:?}", s1);
    let s2 = pkg::read_i686_json(&main2i686(jsonfile))?;
    println!("{:?}", s2);

    return Ok(pkgs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json71() {
        let _ = read_json7("./test/7/web02.json").expect("ERROR");
        assert_eq!(1 + 1, 2);
    }
    #[test]
    fn test_i686() {
        assert_eq!(main2i686("./test/7/web02.json"), "./test/7/web02_i686.json");
    }
}
