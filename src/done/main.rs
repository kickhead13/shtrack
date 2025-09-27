use std::io::prelude::*;

static HELP_MESSAGE: &str = r#"Usage: shtrack-done <TOPIC> <NUMBER> 
Change the <NUMBER>-th to-do status from NONE to DONE in the
specified <TOPIC>.
"#;

fn main()-> Result<(), Box<dyn std::error::Error>> {
    let cli_args: Vec<String> = std::env::args().collect();
    if cli_args.len() > 1 && (cli_args[1] == "--help" || cli_args[1] == "-h") {
        println!("{HELP_MESSAGE}");
    }

    if let Some(home_path) = std::env::home_dir(){
        
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open(format!("{}/.shtrack/{}", home_path.display(), cli_args[1]))
            .unwrap();

        let mut result: Vec<String> = Vec::<String>::new();
        for (indx, line) in std::fs::read_to_string(format!("{}/.shtrack/{}",home_path.display(), cli_args[1])).unwrap().lines().enumerate() {
            if indx == cli_args[2].parse::<usize>()? {
                result.push(format!("DONE{}", &line[4..]));
            } else {
                result.push(line.to_string());
            }
        }
        for line in result {
            if let Err(e) = writeln!(file, "{}", line) {
                eprintln!("Failed to set {} to DONE", &line[4..]);
            }
        }
    }
    Ok(())
}
