// fn ex0() {
//     println!("Hello, world!");
// }

// fn ex1() {
//     use pxls2::ls;
//     match ls::ls("./test/7") {
//         Err(e) => panic!("{:?}", e),
//         Ok(files) => println!("{:#?}", files),
//     }
// }

fn ex2() {
    use pxls2::run;
    if let Err(e) = run::run("test/7", "tmp/7.xlsx") {
        panic!("{:?}", e);
    }
}

fn main() {
    ex2();
}
