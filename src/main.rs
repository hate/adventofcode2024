mod days;
mod helpers;

use days::*;

/**
 * Please don't change this lol - this whole program is probably very broken to breakage
 */
fn main() {
    // get the args from the input (read the README.md)
    let args: Vec<String> = std::env::args().collect();

    if args[1] == "n" {
        new_day::run(args[2].as_str());
        return;
    }

    if !(args[2] == "1" || args[2] == "2") {
        println!("read the README.md");
        return;
    }

    if args.len() > 1 {
        match args[1].as_str() {
            // new days are added here
            "1" => {
                if args[2].as_str() == "1" {
                    one::one::run();
                } else {
                    one::two::run();
                }
            }
            
            "2" => {
                if args[2].as_str() == "1" {
                    two::one::run();
                } else {
                    two::two::run();
                }
            }
            
            "3" => {
                if args[2].as_str() == "1" {
                    three::one::run();
                } else {
                    three::two::run();
                }
            }
            
            "4" => {
                if args[2].as_str() == "1" {
                    four::one::run();
                } else {
                    four::two::run();
                }
            }
            
            "5" => {
                if args[2].as_str() == "1" {
                    five::one::run();
                } else {
                    five::two::run();
                }
            }
            
            "6" => {
                if args[2].as_str() == "1" {
                    six::one::run();
                } else {
                    six::two::run();
                }
            }
            
            "7" => {
                if args[2].as_str() == "1" {
                    seven::one::run();
                } else {
                    seven::two::run();
                }
            }
            _ => println!("re read the README.md perhaps?"),
            
            
            
            
            
            
        }
    } else {
        println!("yeah read the README.md I reckon");
    }
}
