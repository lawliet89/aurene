use clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg, SubCommand};

mod error;

fn make_parser<'a, 'b>() -> App<'a, 'b> {
    let launch = SubCommand::with_name("launch")
        .about(
            "Launch Guild Wars 2 with saved login profiles\n\n\

             Provide directories or files which contains the login profiles. Then, specify the \
             profiles that you would like to launch with the final positional arguments after \
             `-`. Directories are searched with a default glob pattern that can be configured.\n\n\

             The login profiles are obtained after you have logged into Guild Wars 2 with the \
             \"save password\" feature and can be found in the `%APPDATA%\\Guild Wars 2\\Local.dat` \
             path on Windows. You should copy this file and place it in a path to be provided to \
             `aurene`. \n\n\

             To specify the profile to login, you can simply specify the filename. If `aurene` is \
             unable to disambiguate among the login profiles it has discovered (for example due to \
             file names being the same), you would have to provide more compoenents of the \
             absolute path of the profile file.",
        )
        .arg(
            Arg::with_name("auto_chain")
                .help(
                    "When launching multiple profiles, instead of prompting for the user to press \
                     `ENTER` before launching the previous profile, automatically launch the next \
                     profile after the previous profile has exited.",
                )
                .long("auto-chain"),
        )
        .arg(
            Arg::with_name("fail_fast")
                .help(
                    "When launching multiple profiles and any of the profiles fail to launch \
                     all subsequent launches will be cancelled.",
                )
                .long("fail-fast"),
        )
        .arg(
            Arg::with_name("all_profiles")
                .help("Launch all profiles")
                .long("all"),
        )
        .arg(
            Arg::with_name("prompt")
                .help("Prompt for profiles to launch, ignoring all profiles specified in arguments")
                .long("prompt"),
        )
        .arg(
            Arg::with_name("no_restore")
                .help(
                    "Do not restore the original login profile that was present when `aurene` \
                     was first started.")
                .long("no-restore")
        )
        .arg(
            Arg::with_name("glob")
                .long("--glob")
                .help(
                    "Glob pattern to search directories for login profiles. The default will \
                    search all subdirectories recursively for `*.dat` files."
                )
                .long_help("The glob pattern is documented at https://docs.rs/glob/0.2.11/glob/struct.Pattern.html")
                .takes_value(true)
                .default_value("**/*.dat")
        )
        .arg(
            Arg::with_name("path")
            .index(1)
            .help("Specifies paths to directories or files containing the login profiles.")
            .takes_value(true)
            .value_name("path")
            .empty_values(false)
            .multiple(true)
            .required(false)
            .long_help(
            "Provide directories or files which contains the login profiles. Then, specify the \
             profiles that you would like to launch with the final positional arguments after \
             `-`. Directories are searched with a default glob pattern that can be configured.\n\n\

             The login profiles are obtained after you have logged into Guild Wars 2 with the \
             \"save password\" feature and can be found in the `%APPDATA%\\Guild Wars 2\\Local.dat` \
             path on Windows. You should copy this file and place it in a path to be provided to \
             `aurene`. \n\n\

             If no path is provided, the current working directory will be searched."
            )
        )
        .arg(
            Arg::with_name("profile")
                .last(true)
                .help("Specifies profile to launch.")
                .takes_value(true)
                .value_name("profile")
                .empty_values(false)
                .conflicts_with("all_profiles")
                .multiple(true),
        );

        // Profile Path
        // GW2.exe path

    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::InferSubcommands)
        .global_setting(AppSettings::DontCollapseArgsInUsage)
        .global_setting(AppSettings::NextLineHelp)
        .about("Launch Guild Wars 2 with saved profiles")
        .arg(Arg::with_name("fail_silent").long("fail-silent").help(
            "Instead of asking for the user to confirm any failure, \
             exit immediately on failure silently.",
        ))
        .subcommand(launch)
}

fn main() {
    let args = make_parser().get_matches();
}
