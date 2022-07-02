use clap::ArgMatches;

pub fn new_cmd(cmd: &ArgMatches) -> String {
    let lang_attr = if cmd.is_present("typescript") {
        r#" lang="ts""#
    } else {
        ""
    };

    let module_script = if cmd.is_present("module") {
        format!(
            r#"<script context="module"{}>

</script>

"#,
            &lang_attr
        )
    } else {
        String::new()
    };

    let style_tag = if cmd.is_present("style") {
        r#"

<style>

</style>"#
    } else {
        ""
    };

    format!(
        r#"{module_script}\
            <script{lang_attr}>

</script>

<!-- markup here -->{style_tag}"#,
    )
}
