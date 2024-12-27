use clap::{Arg, Command};
use glob::glob;
use shellexpand;

fn main() {
    let matches = Command::new("File Finder")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Finds files based on a pattern")
        .arg(
            Arg::new("pattern")
                .help("The file name pattern to search for")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("extension")
                .help("The file extension to search for")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::new("directory")
                .help("The directory to start searching from")
                .required(false)
                .index(3),
        )
        .get_matches();

    let pattern = matches.get_one::<String>("pattern").map(|s| s.as_str());
    let extension = matches.get_one::<String>("extension").map(|s| s.as_str());
    let directory = matches
        .get_one::<String>("directory")
        .map(|s| s.as_str())
        .unwrap_or(".");

    // Expand the directory path
    let expanded_directory = shellexpand::tilde(directory).to_string();

    let search_pattern = match (pattern, extension) {
        (Some(pat), Some(ext)) if !ext.is_empty() => {
            format!("{}/**/*{}*.{}", expanded_directory, pat, ext)
        }
        (Some(pat), _) => format!("{}/**/*{}*", expanded_directory, pat),
        (None, Some(ext)) if !ext.is_empty() => format!("{}/**/*.{}", expanded_directory, ext),
        (None, _) => format!("{}/**/*", expanded_directory),
    };

    for entry in glob(&search_pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
}
