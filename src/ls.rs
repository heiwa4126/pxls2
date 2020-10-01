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

    struct TestCase {
        path: String,
        wants: Vec<String>,
    }

    fn build_testcase(path: &str, wants: &[&str]) -> TestCase {
        TestCase {
            path: path.to_string(),
            wants: wants.iter().map(|x| x.to_string()).collect(),
        }
    }

    #[test]
    fn test_ls() {
        let cases = [
            build_testcase("./test/1", &["c7", "host1", "R8"]),
            build_testcase("./test/7////", &["R067", "web02"]),
        ];

        for case in cases.iter() {
            match ls(&case.path) {
                Err(e) => panic!("{:?}", e),
                Ok(files) => {
                    println!("{}: {:#?}", case.path, files);
                    assert_eq!(files, case.wants)
                }
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
