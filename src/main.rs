//! Command-line version of the library.
//! 
//! Entering `name_generator -h` will output the help command:
//! ```
//! name_generator
//! ```

use std::env;

use name_maker::name_generator::{RandomNameGenerator, Gender};

fn main() {
    let rng = RandomNameGenerator::init();
    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0);
    let valid_commands = [
        "-h", "--help", "-m", "-f", "--male", "--female", "-M", "-F", "--many", "--family"
    ];

    if args.is_empty() {
        println!("{}", rng.generate());
        return;
    }

    if args.len() == 1 { 
        if let Ok(amount) = args[0].parse::<u32>() {
            generate_many_quick(amount, &rng);
        } else if !valid_commands.contains(&args[0].as_str()) {
            eprintln!("Could not parse the amount of names to be generated.");
            print_command_usage();
        }
    }

    let command: &str = args[0].as_str();

    if valid_commands.contains(&command) {
        let res = match command {
            "-m"|"--male" => generate_specific(Gender::Male, args, rng),
            "-f"|"--female" => generate_specific(Gender::Female, args, rng),
            "-M"|"--many" => generate_many(args, rng),
            "-F"|"--family" => generate_family(args, rng),
            "-h"|"--help" => {
                print_command_usage();
                return;
            }
            _ => {
                eprintln!("Not a valid command.");
                print_command_usage();
                return;
            }
        };

        if let Err(e) = res {
            eprintln!("{}", e);
            print_command_usage();
        }
    }
}

fn generate_specific(gender: Gender, args: Vec<String>, rng: RandomNameGenerator) -> Result<(), &str> {
    match args.len() {
        1 => println!("{}", rng.generate_specific(gender)),
        2 => {
            match args[1].parse::<u32>() {
                Ok(amount) => {
                    let random_names = match gender {
                        Gender::Male => rng.generate_many_specific(amount, 0),
                        Gender::Female => rng.generate_many_specific(0, amount),
                    };
                    match random_names {
                        Some(random_names) => {
                            for name in random_names { println!("{}", name); }
                        }
                        None => return Ok(()),
                    }
                },
                Err(_) => return Err("Could not parse the amount of names to be generated.")
            }
        },
        _ => return Err("Too many command arguments.")
    }

    Ok(())
}

fn generate_many(args: Vec<String>, rng: RandomNameGenerator) -> Result<(), &str> {
    match args.len() {
        0..=1 => return Err("Too few command arguments."),
        2 => {
            let amount = match args[1].parse::<u32>() {
                Ok(amount) => amount,
                Err(_) => return Err("Could not parse the amount of names to be generated.")
            };
            match rng.generate_many(amount) {
                Some(random_names) => {
                    for name in random_names { println!("{}", name); }
                },
                None => return Ok(())
            }
        }
        3 => {
            let male_amount = match args[1].parse::<u32>() {
                Ok(amount) => amount,
                Err(_) => return Err("Could not parse the amount of male names to be generated.")
            };
            let female_amount = match args[2].parse::<u32>() {
                Ok(amount) => amount,
                Err(_) => return Err("Could not parse the amount of female names to be generated.")
            };
            match rng.generate_many_specific(male_amount, female_amount) {
                Some(random_names) => {
                    for name in random_names { println!("{}", name); }
                },
                None => return Ok(())
            }
        }
        4.. => return Err("Too many command arguments."),
        _ => panic!("Should not reach here.")
    }

    Ok(())
}

fn generate_family(args: Vec<String>, rng: RandomNameGenerator) -> Result<(), &str> {
    match args.len() {
        0..=1 => return Err("Too few command arguments."),
        2 => {
            let amount = match args[1].parse::<u32>() {
                Ok(amount) => amount,
                Err(_) => return Err("Could not parse the amount of names to be generated.")
            };

            for name in rng.generate_family(amount) { println!("{}", name); }
        }
        3 => {
            let male_amount = match args[1].parse::<u32>() {
                Ok(amount) => amount,
                Err(_) => return Err("Could not parse the amount of male names to be generated.")
            };
            let female_amount = match args[2].parse::<u32>() {
                Ok(amount) => amount,
                Err(_) => return Err("Could not parse the amount of female names to be generated.")
            };

            for name in rng.generate_family_specific(male_amount, female_amount) { 
                println!("{}", name); 
            }
        }
        4.. => return Err("Too many command arguments."),
        _ => panic!("Should not reach here.")
    }

    Ok(())
}

fn generate_many_quick(amount: u32, rng: &RandomNameGenerator) {
    if amount == 0 { return; }
    
    for name in rng.generate_many(amount).unwrap() {
        println!("{}", name);
    }
}

fn print_command_usage() {
    println!("USAGE:");
    println!("\tname_maker [amount]");
    println!("\tname_maker -m|--male|-f|--female [amount]");
    println!("\tname_maker -M|--many|-F|--family [amount|male_amount female_amount]");
}