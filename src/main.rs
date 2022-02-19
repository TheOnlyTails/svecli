use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::{arg, command};
use owo_colors::{OwoColorize, Stream::Stdout};

mod new;

fn main() {
    let current_dir_path = if let Ok(dir) = env::current_dir() {
        dir
    } else {
        PathBuf::from(".")
    };

    let current_dir = current_dir_path
        .to_str()
        .expect("Could not convert current directory to string, not valid UTF-8");

    let cli = command!("svecli")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)

        .subcommand(command!("new")
            .about("Creates a new Svelte component")
            .arg_required_else_help(true)
            .args(&[
                arg!(<NAME> "the name of the new component").required(true),
                arg!(-p --path "the path to the new component")
                    .default_value(current_dir),
                arg!(-m --module "add a `module=\"context\"` script tag to the component"),
                arg!(-t --typescript "add a `lang=\"ts\"` attribute to the component's script tags"),
                arg!(-s --style "add a style tag to the component"),
                arg!(-f --force "overwrite existing files"),
            ])
        )
        .get_matches();

    if let Some(cmd) = cli.subcommand_matches("new") {
        let formatted_result = new::new_command(cmd);

        let name = cmd.value_of("NAME").unwrap();
        let path_arg = cmd.value_of("path").unwrap();

        let file_path = format!(
            "{path}{name}.svelte",
            path = if cmd.is_present("path") && path_arg != current_dir {
                format!("{path_arg}/")
            } else {
                String::new()
            }
        );

        if !Path::new(&file_path).exists() || cmd.is_present("force") {
            let component_file = File::create(&file_path);

            match component_file {
                Ok(mut created_file) => {
                    created_file.write_all(formatted_result.as_bytes()).unwrap();
                    println!("Created component at {}", &file_path);
                }
                Err(err) => {
                    eprintln!("Could not create component {}.svelte", name.red());
                    eprintln!("Path: {}", &file_path);
                    eprintln!("Caused by {}", err);
                }
            }
        } else {
            eprintln!("Component {}.svelte already exists.", name
                .if_supports_color(Stdout, OwoColorize::red));
        }
    }
}
