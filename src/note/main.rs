use std::io::Write;

static HELP_MESSAGE: &str = r#"Usage: shtrack-note <EDITOR> <NOTE_NAME>
Make a note with a given name.

EDITOR:
    Editor program to be used.
NOTE_NAME:
    Name of the file where the note will be stored.
"#;

static NOTE_MESSAGE: &str = r#"

# This is a shtrack note.
# Write here something of note and you can access it later.
"#;

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let cli_args: Vec<String> = std::env::args().collect();
    if cli_args.len() < 3 {
        print!("{HELP_MESSAGE}");
        std::process::exit(0);
    }

    let program = format!("{}", cli_args[1]);
    if let Some(home_path) = std::env::home_dir() {
        let mut args = Vec::<String>::new();
        args.push(format!("{}/.shtrack/notes/{}", home_path.display(), &cli_args[2]));

        let mut note_buffer = std::fs::File::create(format!("{}/.shtrack/notes/{}", home_path.display(), &cli_args[2]))?;
        let _ = note_buffer.write_all(NOTE_MESSAGE.as_bytes());
        
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
