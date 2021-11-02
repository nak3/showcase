use argh::FromArgs;

fn default_height() -> usize {
    5
}

#[derive(FromArgs)]
/// Reach new heights.
struct GoUp {
    /// whether or not to jump
    #[argh(switch, short = 'j')]
    jump: bool,

    /// how high to go
    #[argh(option, default = "default_height()")]
    height: usize,

    /// an optional nickname for the pilot
    #[argh(option)]
    pilot_nickname: Option<String>,

    /// an optional direction which is "up" by default
    #[argh(option, default = "String::from(\"only up\")")]
    direction: String,
}

fn main() {
    let up: GoUp = argh::from_env();
    println!("height is {}", up.height);

    println!("direction is {}", up.direction);

    if up.jump {
      println!("jump!");
    }

    match up.pilot_nickname {
        None => println!("pilot name is not specified"),
        Some(name) => {
            println!("pilot name is {}", name)
        },
    }
}
