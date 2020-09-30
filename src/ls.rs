// use std::fs;
extern crate glob;
use glob::glob;
use std::error::Error;
use std::path::Path;

pub fn host2file(host: &str, base: &str) -> String {
    return Path::new(base)
        .join(host.to_string() + ".json")
        .display()
        .to_string();
}

pub fn ls(search_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files: Vec<String> = Vec::new();

    // pub fn glob(pattern: &str) -> Result<Paths, PatternError>
    for entry in glob(&(search_path.to_string() + "/*.json"))? {
        match entry {
            Err(e) => return Err(Box::new(e)), // GlobError
            Ok(pathbuf) => {
                let s = pathbuf.as_path().file_stem().unwrap().to_str().unwrap(); // raise panic!
                if !s.ends_with("_i686") {
                    files.push(s.into()); // .to_string()でいいところを.into()使ってみた
                }
            }
        }
    }

    // files.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    files.sort_by_cached_key(|a| a.to_lowercase());
    return Ok(files);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ls() {
        match ls("./test/1") {
            Err(e) => panic!("{:?}", e),
            Ok(files) => {
                print!("{:#?}", files);
                assert_eq!(files, vec!["c7", "host1", "R8"])
            }
        }
        match ls("./test/7") {
            Err(e) => panic!("{:?}", e),
            Ok(files) => {
                print!("{:#?}", files);
                assert_eq!(files, vec!["R067", "web02"])
            }
        }
    }

    #[test]
    fn test_host2file() {
        let s = host2file("R067", "./test/7");
        println!("{}", s);
        assert_eq!(s, "./test/7/R067.json");
    }
}
