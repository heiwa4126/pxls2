extern crate glob;
use anyhow::{anyhow, bail, Result};
use glob::glob;
use std::path::Path;

pub fn host2file(host: impl Into<String>, base: &str) -> String {
    return Path::new(base)
        .join(host.into() + ".json")
        .display()
        .to_string();
}

pub fn ls(search_path: impl Into<String>) -> Result<Vec<String>> {
    let mut files: Vec<String> = Vec::new();

    for entry in glob(&(search_path.into() + "/*.json"))? {
        let path = match entry {
            Err(e) => bail!(e), // GlobError
            Ok(t) => t,
        };
        let s = path
            .file_stem()
            .ok_or_else(|| anyhow!("`{}` has no stem.", path.display()))?
            .to_str()
            .ok_or_else(|| anyhow!("`{}` can't convert to UTF8.", path.display()))?;
        if !s.ends_with("_i686") {
            files.push(s.to_string());
        }
    }
    files.sort_by_cached_key(|a| a.to_lowercase());
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ls() {
        struct TestCase {
            path: String,
            wants: Vec<String>,
        }
        // See https://qiita.com/Kogia_sima/items/6899c5196813cf231054
        // "impl Into<String>" idiom
        fn build_testcase(path: impl Into<String>, wants: &[&str]) -> TestCase {
            TestCase {
                path: path.into(),
                wants: wants.iter().map(std::string::ToString::to_string).collect(),
            }
        }

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
