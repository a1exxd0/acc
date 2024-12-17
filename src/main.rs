use acc::parse_options::types::AccArgs;

fn main() {
    let args = AccArgs::from_args();

    println!("{:?}", args);
}
