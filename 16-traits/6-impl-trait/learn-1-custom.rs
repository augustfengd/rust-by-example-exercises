use std::fmt;

fn x<R: fmt::Debug>(r: R) {
    println!("{:?}", r);
}

fn y(r: impl fmt::Debug) {
    println!("{:?}", r);
}

fn main() {
    let foobar = String::from("helloworld");

    x(&foobar);
    y(&foobar);
}
