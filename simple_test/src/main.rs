#[derive(Debug, Eq, PartialEq)]
struct Tmp {
    val: i32,
    name: String,
}

fn main() {
    let a = Tmp {
        val: 12,
        name: String::from("aaa"),
    };
    println!("{}", a.name);
}