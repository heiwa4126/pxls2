// fn ex0() {
//     println!("Hello, world!");
// }

fn ex1() {
    use pxls2::ls;
    match ls::ls("./test/7") {
        Err(e) => panic!("{:?}", e),
        Ok(files) => println!("{:#?}", files),
    }
}

fn main() {
    ex1();
}
