static HELP_MESSAGE: &str = r#"Usage: shtrack-list 
List all todos from all topics.
"#;

fn main() {
    let cli_args: Vec<String> = std::env::args().collect();
    if cli_args.len() > 1 {
        println!("{HELP_MESSAGE}");
        std::process::exit(0);
    }

    if let Some(home_path) = std::env::home_dir() {
        let paths = std::fs::read_dir(format!("{}/.shtrack/", home_path.display())).unwrap();

        for path in paths {
            let unwrap_path = path.unwrap();
            let filepath = unwrap_path.path();
            
            let short_filepath = unwrap_path.file_name();
            let topic = short_filepath.to_str().unwrap();
            println!("{topic}");

            if short_filepath.into_string().expect("COULD NOT OBTAIN STRING FROM OSSTRING").starts_with("SHTRACK_TOPIC.") {
                for (count, line) in std::fs::read_to_string(filepath).unwrap().lines().enumerate() {
                    if line[0..4] == *"NONE" {
                        println!(" {}: [ ] {}", count, &line[4..]);
                    } else if line[0..4] == *"DONE" {
                        println!(" {}: [X] {}", count, &line[4..]);
                    }
                }
            }
            
        }
    }
}
