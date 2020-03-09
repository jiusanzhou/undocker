use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("hxer")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(vec![
            SubCommand::with_name("undocker")
                .about("处理未容器化的项目列表")
                .args(&[
                    Arg::with_name("file")
                        .short("f")
                        .takes_value(true)
                        .help("CSV文件"),
                ]),
        ])
}