use clap::Command;

/// Representation of compiler args in a nice format.
#[derive(Debug)]
pub struct AccArgs {
    pub output_path: String,
    pub input_files: Vec<String>,
    pub opt_level: u8,
    pub link_dirs: Vec<String>,
    pub include_dirs: Vec<String>,
    pub c_std: u16,
}

/// Wrapper type for command to perform custom actions and build
/// on API.
///
/// # Examples
/// ```rust
/// // You can also call `get_matches()` at the end to retieve the clap matches type
/// use acc::parse_options::types;
/// let acc_command = types::AccCommand::base()
///     .arg_output_path()
///     .arg_input_files()
///     .arg_opt_level()
///     .arg_link_dirs()
///     .arg_include_dirs()
///     .arg_c_std();
/// ```
pub struct AccCommand(pub Command);
