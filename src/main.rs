use std::{env, fs, process::ExitCode};
use vyron::{check, lower_to_ir};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let command = args.next();
    let path = args.next();

    if !matches!(command.as_deref(), Some("check" | "ir")) || path.is_none() {
        eprintln!("VYRON 0.1.0");
        eprintln!("Usage: vyron <check|ir> <file.vyr>");
        return ExitCode::from(2);
    }

    let command = command.unwrap();
    let path = path.unwrap();
    let source = match fs::read_to_string(&path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("vyron: cannot read '{}': {}", path, error);
            return ExitCode::FAILURE;
        }
    };

    match command.as_str() {
        "check" => match check(&source) {
            Ok(module) => {
                println!("✓ {}: {} function(s), {} declared effect(s)",
                    path, module.functions.len(), module.effects.len());
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("vyron: {}", error);
                ExitCode::FAILURE
            }
        },
        "ir" => match lower_to_ir(&source) {
            Ok(module) => {
                println!("{:#?}", module);
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("vyron: {}", error);
                ExitCode::FAILURE
            }
        },
        _ => ExitCode::from(2),
    }
}
