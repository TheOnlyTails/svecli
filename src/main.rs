use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::{arg, command};
use owo_colors::{OwoColorize, Stream::Stdout};

use endpoint::endpoint_cmd;
use new::new_cmd;

mod new;
mod endpoint;

fn main() {
    let current_dir_path = if let Ok(dir) = env::current_dir() {
        dir
    } else {
        PathBuf::from(".")
    };

    let current_dir = current_dir_path
        .to_str()
        .expect("Valid UTF-8 directory");

    let cli = command!("svecli")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)

        .subcommand(command!("new")
            .about("Creates a new Svelte component")
            .arg_required_else_help(true)
            .args(&[
                arg!(<NAME> "the name of the new component"),
                arg!(-p --path "the path to the new component")
                    .default_value(current_dir),
                arg!(-m --module "add a `module=\"context\"` script tag to the component"),
                arg!(-t --typescript "add a `lang=\"ts\"` attribute to the component's script tags"),
                arg!(-s --style "add a style tag to the component"),
                arg!(-f --force "overwrite existing files"),
            ])
        )
        .subcommand(command!("endpoint")
            .about("Creates a new SvelteKit endpoint")
            .arg_required_else_help(true)
            .args(&[
                arg!(<NAME> "the name of the new endpoint"),
                arg!(-p --path "the path to the new endpoint")
                    .default_value(current_dir),
                arg!(--"no-get" "don't create a get endpoint"),
                arg!(-e --extension "set the file extension for the endpoint")
                    .default_value("json"),
                arg!(--post "add a post endpoint"),
                arg!(--put "add a put endpoint"),
                arg!(--patch "add a patch endpoint"),
                arg!(--delete "add a delete endpoint"),
                arg!(-t --typescript "enable typescript"),
                arg!(-f --force "overwrite existing files"),
            ])
        )
        .get_matches();

    match cli.subcommand() {
        Some(("new", cmd)) => {
            let cmd_result = new_cmd(cmd);

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
                        created_file.write_all(cmd_result.as_bytes()).unwrap();
                        println!("Created component at {}", &file_path);
                    }
                    Err(err) => {
                        eprintln!("Could not create component {}.svelte", name.red());
                        eprintln!("Path: {}", &file_path);
                        eprintln!("Caused by {}", err);
                    }
                }
            } else {
                eprintln!(
                    "Component {}.svelte already exists.",
                    name.if_supports_color(Stdout, OwoColorize::red)
                );
            }
        }
        Some(("endpoint", cmd)) => {
            let cmd_result = endpoint_cmd(cmd);

            let name = cmd.value_of("NAME").unwrap();
            let path_arg = cmd.value_of("path").unwrap();

            let js_ext = if cmd.is_present("typescript") {
                "ts"
            } else {
                "js"
            };

            let endpoint_ext = if !cmd.is_present("extension") {
                cmd.value_of("extension").unwrap().to_string() + "."
            } else {
                String::new()
            };
            let ext = endpoint_ext + js_ext;

            let file_path = format!(
                "{path}{name}.{ext}",
                path = if cmd.is_present("path") && path_arg != current_dir {
                    format!("{path_arg}/")
                } else {
                    String::new()
                }
            );

            if !Path::new(&file_path).exists() || cmd.is_present("force") {
                let endpoint_file = File::create(&file_path);

                match endpoint_file {
                    Ok(mut created_file) => {
                        created_file.write_all(cmd_result.as_bytes()).unwrap();
                        println!("Created endpoint at {}", &file_path);
                    }
                    Err(err) => {
                        eprintln!("Could not create endpoint {}.{}", name.red(), ext);
                        eprintln!("Path: {}", &file_path);
                        eprintln!("Caused by {}", err);
                    }
                }
            } else {
                eprintln!(
                    "Endpoint {}.{} already exists.",
                    name.if_supports_color(Stdout, OwoColorize::red),
                    ext
                );
            }
        }
        _ => unreachable!()
    }
}
