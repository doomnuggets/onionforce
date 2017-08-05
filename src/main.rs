extern crate rand; // For picking random chars from the given charset.
use rand::Rng;

extern crate getopts; // For parsing commandline arguments.
use getopts::Options;

use std::env;

/// Randomly generates a domain.
fn generate_random_domain(domain_length: u8) -> String {
    let charset = ["a", "b", "c", "d", "e", "f", "g", "h",
                   "i", "j", "k", "l", "m", "n", "o", "p",
                   "q", "r", "s", "t", "u", "v", "w", "x",
                   "y", "z", "2", "3", "4", "5", "6", "7"];
    let mut domain_buffer = String::new();
    for _ in 0..domain_length {
        let mut rng = rand::thread_rng();
        match rng.choose(&charset) {
            Some(random_char) => domain_buffer += random_char,
            _ => println!("Failed to generate a random character."),
        };
    }
    domain_buffer
}

/// Prints the usage of the program.
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [-t onion | -c 10 ]", program);
    println!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu"); // Don't accept values for the one time help display (optflag).
    opts.optopt("t", "tld", "which toplevel domain name", "onion"); // Accept values for -t / --tld option (optopt).
    opts.optopt("c", "count", "how many domains to generate", "10");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    // Display the help message and exit.
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // Abort execution when there are unparsed arguments.
    if !matches.free.is_empty() {
        print_usage(&program, opts);
        return;
    }

    // determine the number of domains to generate.
    let number_of_domains = match matches.opt_str("count") {
        Some(s) => {
            match s.parse::<u64>() {
                Ok(c) => c,
                Err(e) => {
                    println!("Unable to parse --count: {}", e);
                    return;
                }
            }
        }
        None => 10,
    };

    // Determine the toplevel domain.
    let tld = match matches.opt_str("tld") {
        Some(s) => s,
        None => "onion".to_string(),
    };

    // Generate the desired number of random domains.
    for _ in 0..number_of_domains {
        let domain = generate_random_domain(16u8);
        println!("{}.{}", domain, tld);
    }
}
