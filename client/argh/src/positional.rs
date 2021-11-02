use argh::FromArgs;

#[derive(FromArgs)]
/// Demonstration for positional arguments.
struct PositionalDemo {
    #[argh(positional)]
    first: String,

    #[argh(positional)]
    others: Vec<String>,
}

fn main() {
    let demo: PositionalDemo = argh::from_env();

    println!("the first arg is {}", demo.first);
    println!("the other argument is {:?}", demo.others);
}
