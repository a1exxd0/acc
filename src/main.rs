use acc::parse_options::types::AccArgs;
use acc_compiler::preprocessor::character_mapping::pretty_print;
use acc_compiler::preprocessor::character_mapping::{CharMapper, MappedChar};

fn main() -> std::io::Result<()> {
    let args = AccArgs::from_args();

    println!("{:?}", args);

    for input_file in &args.input_files {
        let content = std::fs::read_to_string(input_file)?;
        let mapper = CharMapper::new(&content);

        println!(
            "{}:\n{}\n",
            input_file,
            pretty_print::mapped_chars_to_str(&mapper.into_iter().collect::<Vec<MappedChar>>()),
        );
    }

    Ok(())
}
