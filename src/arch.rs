use lazy_static::lazy_static;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Arch(u16);

#[derive(Error, Debug)]
pub enum ArchError {
    #[error("`{0}` cant parse as Arch.")]
    ParseError(String),
    #[error("`{0}` has no period.")]
    NoPriod(String),
}

impl Clone for Arch {
    #[inline]
    fn clone(&self) -> Arch {
        Arch(self.0)
    }
}

use std::fmt;
impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_s())
    }
}

use std::str::FromStr;
impl FromStr for Arch {
    type Err = ArchError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::from_s(s) {
            Some(x) => Ok(*x),
            None => Err(ArchError::ParseError(s.into())),
        }
    }
}

lazy_static! {
    static ref CNVTBL: (
        HashMap<&'static str, Arch>,	// 0: &str -> arch
        HashMap<Arch, &'static str>,  // 1: Arch -> &str
    ) = {
        let mut f = HashMap::new();
        let mut t = HashMap::new();
        for v in &[
            (Arch::X86_64, "x86_64"),
            (Arch::I686, "i686"),
            (Arch::NOARCH, "noarch"),
        ] {
            f.insert(v.1, v.0);
            t.insert(v.0, v.1);
        }
        (f, t)
    };
}

impl Arch {
    pub const X86_64: Arch = Arch(0);
    pub const I686: Arch = Arch(1);
    pub const NOARCH: Arch = Arch(2);

    pub fn to_s(&self) -> &str {
        CNVTBL.1.get(self).unwrap()
    }

    // associates

    pub fn from_s(s: &str) -> Option<&Self> {
        CNVTBL.0.get(s)
    }

    pub fn from_ends(s: &str) -> Result<&Self, ArchError> {
        let i = s
            .rfind('.')
            .ok_or_else(|| ArchError::NoPriod(s.to_string()))?;
        Self::from_s(&s[i + 1..]).ok_or_else(|| ArchError::ParseError(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn typename<T>(_: T) -> &'static str {
        std::any::type_name::<T>()
    }

    #[test]
    fn test_arch() {
        // let i686 = Arch::I686;
        let i686 = Arch::from_s("i686").unwrap().clone();
        let x86 = FromStr::from_str("x86_64").expect("ERROR!");
        println!("{:?}", &i686);
        println!("{}", &i686);

        use std::cmp::Ordering;
        assert!(i686 > x86);
        assert!(i686 < Arch::NOARCH);
        assert_eq!(i686.cmp(&x86), Ordering::Greater);
        assert_eq!(i686.cmp(&Arch::NOARCH), Ordering::Less);
    }

    #[test]
    fn test_archerr() {
        match Arch::from_str("unknown") {
            Ok(_) => panic!("ERROR"),
            Err(e) => println!("{}-> {}", typename(&e), e),
        }
    }

    #[test]
    fn test_arch_fromends() {
        let rc = Arch::from_ends("test.i686").unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(rc, &Arch::I686);

        match Arch::from_ends("test.z80").unwrap_err() {
            ArchError::ParseError(s) => println!("{}", s),
            e => panic!("ERROR! {}", e),
        }
        match Arch::from_ends("test").unwrap_err() {
            ArchError::NoPriod(s) => println!("{}", s),
            e => panic!("ERROR! {}", e),
        }

        // assert_eq!(Arch::from_ends("test.i386"), Err(ArchError::ParseError));
    }
    #[test]
    fn test_arch_copy() {
        let a = Arch::I686;
        let b = a; // copy semantic
        let c = a.clone();
        assert_eq!(b, a);
        assert_eq!(b, c);
    }
}
