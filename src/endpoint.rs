use clap::ArgMatches;

pub fn endpoint_command(cmd: &ArgMatches) -> String {
    let typedefs = if cmd.is_present("typescript") {
        "import type { RequestHandler } from \"@sveltejs/kit\";\n\n"
    } else {
        ""
    };

    let endpoint = |name: &str| -> String {
        if cmd.is_present("typescript") {
            format!("\n\
            \n\
            export const {name}: RequestHandler = async () => {{\n\
            \t\n\
            }};")
        } else {
            format!("\n\
            /** @type {{import('@sveltejs/kit').RequestHandler}} */\n\
            export async function {}() {{\n\
            \t\n\
            }};\n", name)
        }
    };

    let get_endpoint = if !cmd.is_present("no-get") {
        endpoint("get")
    } else {
        String::new()
    };

    let post_endpoint = if cmd.is_present("post") {
        endpoint("post")
    } else {
        String::new()
    };

    let put_endpoint = if cmd.is_present("put") {
        endpoint("put")
    } else {
        String::new()
    };

    let patch_endpoint = if cmd.is_present("patch") {
        endpoint("patch")
    } else {
        String::new()
    };

    let delete_endpoint = if cmd.is_present("delete") {
        endpoint("delete")
    } else {
        String::new()
    };

    let result = format!("{get_endpoint}\
     {post_endpoint}\
     {put_endpoint}\
     {patch_endpoint}\
     {delete_endpoint}\
    ");

    match result.strip_prefix('\n') {
        Some(result_stripped) => String::from(typedefs) + result_stripped,
        None => result
    }
}