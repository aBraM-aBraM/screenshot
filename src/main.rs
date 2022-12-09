use clap::Parser;

/// Takes a screenshot
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to save the screenshot to
    #[arg(short, long, default_value_t = home::home_dir().unwrap().to_str().unwrap().to_string())]
    screenshot_dir: String,
}

fn main() {
    let args = Args::parse();
    dbg!(args.screenshot_dir);
    println!("Hello, world!");
}
