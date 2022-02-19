use clap::ArgMatches;

pub fn new_command(cmd: &ArgMatches) -> String {
    let lang_attr = if cmd.is_present("typescript") {
        r#" lang="ts""#
    } else {
        ""
    };

    let module_script = if cmd.is_present("module") {
        format!(
            "<script context=\"module\"{}>\n\
            \n\
            </script>\
            \n\n",
            &lang_attr
        )
    } else {
        String::new()
    };

    let style_tag = if cmd.is_present("style") {
        "\n\n<style>\n\n</style>"
    } else {
        ""
    };

    format!(
        "{module_script}\
            <script{lang_attr}>\n\
            \n\
            </script>\n\
            \n\
            <!-- markup here -->\
            \
            {style_tag}",
    )
}
