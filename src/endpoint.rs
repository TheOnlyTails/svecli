use clap::ArgMatches;

pub fn endpoint_cmd(cmd: &ArgMatches) -> String {
    let typedefs = if cmd.is_present("typescript") {
        let name = cmd.value_of("NAME").unwrap();
        format!(
            r#"import type {{ RequestHandler }} from "./{name}";

"#
        )
    } else {
        String::new()
    };

    let endpoint = |name| {
        if cmd.is_present("typescript") {
            format!(
                r#"

export const {name}: RequestHandler = async () => {{

}};"#
            )
        } else {
            format!(
                r#"
/** @type {{import('./{filename}').RequestHandler}} */
export async function {name}() {{

}};
"#,
                filename = cmd.value_of("NAME").unwrap()
            )
        }
    };

    let get_endpoint = if !cmd.is_present("no-get") {
        endpoint("GET")
    } else {
        String::new()
    };

    let post_endpoint = if cmd.is_present("post") {
        endpoint("POST")
    } else {
        String::new()
    };

    let put_endpoint = if cmd.is_present("put") {
        endpoint("PUT")
    } else {
        String::new()
    };

    let patch_endpoint = if cmd.is_present("patch") {
        endpoint("PATCH")
    } else {
        String::new()
    };

    let delete_endpoint = if cmd.is_present("delete") {
        endpoint("DELETE")
    } else {
        String::new()
    };

    let result = format!(
        "{get_endpoint}\
     {post_endpoint}\
     {put_endpoint}\
     {patch_endpoint}\
     {delete_endpoint}\
    "
    );

    match result.strip_prefix('\n') {
        Some(result_stripped) => String::from(typedefs) + result_stripped,
        None => result,
    }
}
