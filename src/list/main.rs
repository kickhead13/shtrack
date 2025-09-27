static HELP_MESSAGE: &str = r#"Usage: shtrack-list 
List all todos from all topics.
"#;

fn main() {
    let cli_args: Vec<String> = std::env::args().collect();
    if cli_args.len() > 1 && (cli_args[1] == "--help" || cli_args[1] == "-h") {
        println!("{HELP_MESSAGE}");
    }

    if let Some(home_path) = std::env::home_dir() {
        let paths = std::fs::read_dir(format!("{}/.shtrack/", home_path.display())).unwrap();

        for path in paths {
            let unwrap_path = path.unwrap();
            let filepath = unwrap_path.path();
            let filename = filepath.display();
            
            let short_filepath = unwrap_path.file_name();
            let topic = short_filepath.to_str().unwrap();
            println!("{topic}");

            for line in std::fs::read_to_string(filepath).unwrap().lines() {
                if line[0..4] == *"NONE" {
                    println!("  [ ] {}", &line[4..]);
                } else if line[0..4] == *"DONE" {
                    println!("  [X] {}", &line[4..]);
                }
            }
            
        }
    }
}
