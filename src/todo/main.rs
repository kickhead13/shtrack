use std::fs::OpenOptions;
use std::io::prelude::*;

static HELP_MESSAGE: &str = r#"Usage: shtrack-todo <TOPIC> <Description of TODO...>
Create topic to keep shtrack of.

TOPIC:
    Name of the topic in which to include this todo.
DESCRIPTION:
    A sentence describing what it is to do.

Examples:

Add a new to-do:
.----------------------------------------.
| $ shtrack todo shtrack Test shtrack.   |
.________________^^^^^^^__^^^^^^^^^^_____.
                 TOPIC        Description of what there is to do.        
"#;


fn main() {
    let cli_args: Vec<String> = std::env::args().collect();
    if cli_args.len() == 1 || cli_args[1] == "--help" || cli_args[1] == "-h" {
        println!("{HELP_MESSAGE}");
    }

    if let Some(path) = std::env::home_dir() {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}/.shtrack/{}", path.display(), cli_args[1]))
            .unwrap();
        
        if let Err(e) = writeln!(file, "{}", cli_args[2..].join(" ")) {
            eprintln!("Topic {} does not exist.", cli_args[1]);
        }
    }
}
