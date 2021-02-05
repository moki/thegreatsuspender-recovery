use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path;
use std::process;

const EXECUTABLE: &str = "rust";

const DELIMETER: &str = "&uri=";

fn print_usage() {
    println!(
        "usage:\n{} path/to/the/session.txt [path/to/the/output_file.txt] [delimeter]\n\
        second argument being optional, defaults to url.txt relative to the cwd\n\
        third argument being optionsal, default to the &uri=",
        EXECUTABLE
    );
}

fn parse_config(args: &[String]) -> (&str, &str, &str) {
    match args.len() {
        4 => return (&args[1], &args[2], &args[3]),
        3 => return (&args[1], &args[2], DELIMETER),
        2 => return (&args[1], &"urls.txt", DELIMETER),
        _ => return (&"", &"", &""),
    }
}

fn transform<'s>(session: &'s str, delimeter: &str) -> Vec<&'s str> {
    let mut urls = Vec::new();

    for line in session.lines() {
        urls.push(if line.contains(delimeter) {
            let i = line.find(delimeter).unwrap_or(0);
            &line[i + delimeter.len()..line.len()]
        } else {
            &line
        });
    }

    urls
}

fn process_session(input: &str, output: &str, delimeter: &str) -> Result<(), Box<dyn Error>> {
    let session = fs::read_to_string(input)?;

    let transformed = transform(&session, &delimeter);

    let output = path::Path::new(output);
    let display = output.display();

    let mut file = fs::File::create(&output).expect(&format!("failed create {}", display));

    file.write_all(transformed.join(&"\n").as_bytes())
        .expect(&format!("failed write {}", display));

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (input, output, delimeter) = parse_config(&args);

    if input.len() == 0 || output.len() == 0 || delimeter.len() == 0 {
        print_usage();

        process::exit(1);
    }

    if input == "--help" || input == "-h" {
        print_usage();

        process::exit(1);
    }

    if let Err(e) = process_session(&input, &output, &delimeter) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
