/*We’ll make our own version of the classic
command line search tool grep (globally search a regular expression
and print). In the simplest use case, grep searches a specified file for
a specified string. To do so, grep takes as its arguments a file path
and a string. Then it reads the file, finds lines in that file that contain
the string argument, and prints those lines.*/

/*We run our program with cargo run, two hyphens to
indicate the following arguments are for our program rather than for
cargo, a string to search for, and a path to a file to search in, like so:

$ cargo run -- searchstring example-filename.txt*/

use std::env;
use std::process;

use Project_command_line::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    //run(config);

    if let Err(e) = Project_command_line::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

