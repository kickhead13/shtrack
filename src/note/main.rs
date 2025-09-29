use std::io::Write;

static HELP_MESSAGE: &str = r#"Usage: shtrack-note <EDITOR> <NOTE_NAME>
Make a note with a given name.

EDITOR:
    Editor program to be used.
NOTE_NAME:
    Name of the file where the note will be stored.
"#;

static NOTE_MESSAGE: &str = r#"

### This is a shtrack note.
### Write here something of note and you can access it later.
"#;


fn list_notes() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(home_path) = std::env::home_dir() {
        let paths = std::fs::read_dir(format!("{}/.shtrack/notes", home_path.display())).unwrap();

        for path in paths {
            let unwrap_path = path.unwrap();
            let filepath = unwrap_path.path();
        
            let short_filepath = unwrap_path.file_name();
            let topic = short_filepath.to_str().unwrap();
            println!("{topic}");
        }
    }
    Ok(())
} 

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let cli_args: Vec<String> = std::env::args().collect();
    if cli_args[1] == "--help" || cli_args[1] == "-h" {
        print!("{HELP_MESSAGE}");
        std::process::exit(0);
    }

    if cli_args[1] == "list" {
        return list_notes();
    }

    let program = std::env::var("EDITOR").expect("vi"); 
        // format!("{}", cli_args[1]);
    if let Some(home_path) = std::env::home_dir() {
        let mut args = Vec::<String>::new();
        args.push(format!("{}/.shtrack/notes/{}", home_path.display(), &cli_args[1]));

        if let Ok(exists) = std::fs::exists(format!("{}/.shtrack/notes/{}", home_path.display(), &cli_args[1])) {
            if ! exists {
                let mut note_buffer = std::fs::File::create(format!("{}/.shtrack/notes/{}", home_path.display(), &cli_args[1]))?;    
                let _ = note_buffer.write_all(NOTE_MESSAGE.as_bytes());
            }
        }
        
        match std::process::Command::new(program).args(args).status() {
            Err(e) => {
                eprintln!("{}", e);
                print!("{HELP_MESSAGE}");
                std::process::exit(-1);
            },
            Ok(_) => {
               println!("Note added!");
            }
        }
    }
    Ok(())
}
