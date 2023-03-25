use i3_util_lib::Config;

fn main() {
    let config = Config { name: String::from("World") };
    println!("Hello, {}", config.name);
}
