use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 && args[1] == "init" && args[2] == "bash" {
        print!(". <(curl -sSL https://gist.githubusercontent.com/luisadha/5848e6b6af43cde1867cbea06ccabe2f/raw)");
        exit(0);
    }

    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        println!("Help messages.");
        exit(0);
    }

    println!("Usage: casex init bash");
    exit(1);
}
