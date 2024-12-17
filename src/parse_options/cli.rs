use crate::parse_options::types::AccArgs;
use crate::parse_options::types::AccCommand;
use clap::builder::styling::{AnsiColor, Color, Style};
use clap::builder::Styles;
use clap::{crate_name, crate_version, Arg, ArgAction, Command};

impl AccCommand {
    /// Base constructor for an AccCommand object. Sets up formatting for
    /// the project binary output as well as obligatory setup for `clap`
    ///
    /// # Example
    /// ```rust
    /// use acc::parse_options::types::AccCommand;
    /// let command = AccCommand::base();
    /// ```
    pub fn base() -> Self {
        let help_template = r#"
            {name} {version}
            {author-with-newline}
            {about-with-newline}
            {usage-heading} {usage}
            {all-args}"#
            .trim()
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join("\n");

        AccCommand(
            Command::new(crate_name!())
                .author("Alex Do")
                .version(crate_version!())
                .about("C90 compiler in Rust. ")
                .long_about("C90 compiler in Rust. ")
                .styles(STYLES)
                .help_template(help_template)
                .color(clap::ColorChoice::Auto)
                .arg_required_else_help(true)
                .allow_external_subcommands(true),
        )
    }

    /// Appends output path command for `acc`. This should be called before
    /// calling for matches typically, but we use the builder pattern to
    /// allow for flexibility.
    ///
    /// Only one such output path is allowed, specifying file path to output.
    ///
    /// # Example
    /// ```rust
    /// use acc::parse_options::types::AccCommand;
    /// let command = AccCommand::base()
    ///     .arg_output_path();
    /// ```
    pub fn arg_output_path(self) -> Self {
        self.arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .default_value("main")
                .help("Place output into <name>")
                .num_args(1),
        )
    }

    /// Appends input paths for `acc`. This should be called before
    /// calling for matches typically, but we use the builder pattern to
    /// allow for flexibility.
    ///
    /// Inputs are specified without flags, and at least 1 is required.
    ///
    /// # Example
    /// ```rust
    /// use acc::parse_options::types::AccCommand;
    /// let command = AccCommand::base()
    ///     .arg_input_files();
    /// ```
    pub fn arg_input_files(self) -> Self {
        self.arg(
            Arg::new("input")
                .help("Input source files")
                .action(ArgAction::Append)
                .required(true),
        )
    }

    /// Appends optimization level command for `acc`.
    ///
    /// Only one level is allowed.
    ///
    /// # Example
    /// ```rust
    /// use acc::parse_options::types::AccCommand;
    /// let command = AccCommand::base()
    ///     .arg_opt_level();
    /// ```
    pub fn arg_opt_level(self) -> Self {
        self.arg(
            Arg::new("opt_level")
                .short('O')
                .default_value("0")
                .help("Optimization level in [0..=3]")
                .value_parser(clap::value_parser!(u8).range(0..=3))
                .num_args(1),
        )
    }

    /// Appends library directory paths for `acc`.
    ///
    /// Not a required item. Can have multiple.
    ///
    /// # Example
    /// ```rust
    /// use acc::parse_options::types::AccCommand;
    /// let command = AccCommand::base()
    ///     .arg_link_dirs();
    /// ```
    pub fn arg_link_dirs(self) -> Self {
        self.arg(
            Arg::new("link_dirs")
                .short('L')
                .help("Specify directory containing libraries to link"),
        )
    }

    /// Appends include directory paths for `acc`.
    ///
    /// Not a required item. Can have multiple.
    ///
    /// # Example
    /// ```rust
    /// use acc::parse_options::types::AccCommand;
    /// let command = AccCommand::base()
    ///     .arg_include_dirs();
    /// ```
    pub fn arg_include_dirs(self) -> Self {
        self.arg(
            Arg::new("include_dirs")
                .short('I')
                .help("Specify directory containing headers"),
        )
    }

    /// Appends cstd command for `acc`.
    ///
    /// Only one standard is allowed. Currently supports "c90" only.
    /// Case sensitive.
    ///
    /// # Example
    /// ```rust
    /// use acc::parse_options::types::AccCommand;
    /// let command = AccCommand::base()
    ///     .arg_c_std();
    /// ```
    pub fn arg_c_std(self) -> Self {
        self.arg(
            Arg::new("c_std")
                .long("std")
                .help("Choose C standard")
                .require_equals(true)
                .num_args(1)
                .default_value("c90")
                .value_parser(SUPPORTED_C_VERSIONS),
        )
    }

    fn arg(self, a: impl Into<Arg>) -> Self {
        AccCommand(self.0.arg(a))
    }

    pub fn get_matches(self) -> clap::ArgMatches {
        self.0.get_matches()
    }
}

impl AccArgs {
    /// Interpret arguments from command line recieved according to
    /// `AccArgs` and `AccCommand` specification.
    ///
    /// ```no_run rust
    /// use acc::parse_options::types::AccArgs;
    /// let args: AccArgs = AccArgs::from_args();
    /// ```
    pub fn from_args() -> Self {
        let matches = AccCommand::base()
            .arg_output_path()
            .arg_input_files()
            .arg_opt_level()
            .arg_link_dirs()
            .arg_include_dirs()
            .arg_c_std()
            .get_matches();

        Self {
            output_path: matches.get_one::<String>("output").cloned().unwrap(),
            input_files: matches
                .get_many::<String>("input")
                .map(|items| items.cloned().collect())
                .unwrap(),
            opt_level: *matches.get_one::<u8>("opt_level").unwrap(),
            link_dirs: matches
                .get_many::<String>("link_dirs")
                .map(|items| items.cloned().collect())
                .unwrap_or_default(),
            include_dirs: matches
                .get_many::<String>("include_dirs")
                .map(|items| items.cloned().collect())
                .unwrap_or_default(),
            c_std: matches
                .get_one::<String>("c_std")
                .map(|std| std[1..].parse().unwrap())
                .unwrap(),
        }
    }
}

/// Currently supported C version. Leaving room for expansion down the line
const SUPPORTED_C_VERSIONS: [&str; 1] = ["c90"];

/// Styles used for the `clap` toolkit
const STYLES: Styles = Styles::styled()
    .usage(
        Style::new()
            .bold()
            .underline()
            .fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
    )
    .header(
        Style::new()
            .bold()
            .underline()
            .fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
    )
    .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))))
    .invalid(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Red))))
    .error(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Red))))
    .valid(
        Style::new()
            .bold()
            .underline()
            .fg_color(Some(Color::Ansi(AnsiColor::Green))),
    )
    .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::White))));
