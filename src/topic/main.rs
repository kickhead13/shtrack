static HELP_MESSAGE: &str = r#"Usage: shtrack-topic <NAME>
Create topic to keep shtrack of.

NAME:
    Name of your topic.

Example
.-------------------------------------.
| $ shtrack topic my-first-topic      |
._____________________________________.
"#;

fn main() {
    let cli_args: Vec<String> = std::env::args().collect();
    if cli_args.len() == 1 || cli_args[1] == "--help" || cli_args[1] == "-h" {
        println!("{HELP_MESSAGE}");
    }

    if let Some(path) = std::env::home_dir() {
        if let Err(_) = std::fs::exists(format!("{}/.shtrack", path.display())) {
            if let Err(_) = std::fs::create_dir(format!("{}/.shtrack", path.display())) {
                eprintln!("Cannot create ~/.shtrack directory...");  
                std::process::exit(-1);
            }
        }

        if let Ok(_) = std::fs::File::create(format!("{}/.shtrack/{}", path.display(), &cli_args[1])) {
            println!("Created new topic {}.", &cli_args[1]);
            std::process::exit(0);
        }
    }
}
