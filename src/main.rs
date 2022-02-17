use std::env;
use std::path::PathBuf;

use clap::{arg, command};

fn main() {
    let current_dir_path = if let Ok(dir) = env::current_dir() {
        dir
    } else {
        PathBuf::from(".")
    };

    let current_dir = current_dir_path
        .to_str()
        .expect("Could not convert current directory to string, not valid UTF-8");

    let cli = command!("svelte")
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
            ])
        )
        .get_matches();

    if let Some(new_command) = cli.subcommand_matches("new") {
        let name = new_command.value_of("NAME").unwrap();
        let path = new_command.value_of("path").unwrap();

        let lang_attr = if new_command.is_present("typescript") {
            r#" lang="ts""#
        } else {
            ""
        };

        let module_script = if new_command.is_present("module") {
            format!("<script context=\"module\"{}>\n\
            \n\
            </script>\
            \n\n", &lang_attr)
        } else {
            String::new()
        };

        let style_tag = if new_command.is_present("style") {
            "\n\n<style>\n\n</style>"
        } else {
            ""
        };

        let formatted_result = format!(
            "<!-- {path}/{name}.svelte -->\n\
            {module_script}\
            <script{lang_attr}>\n\
            \n\
            </script>\n\
            \n\
            <!-- markup here -->\
            \
            {style_tag}",
        );

        println!("{}", formatted_result.as_str());
    }
}
