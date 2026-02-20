use {
    camino::Utf8PathBuf,
    cargo_metadata::MetadataCommand,
    proc_macro::TokenStream,
    serde::Deserialize,
    std::{fs, path::Path},
    syn::{LitStr, parse_macro_input},
};

#[derive(Deserialize)]
pub struct InstructionFn {
    pub name: String,
    pub args: Vec<String>,
}

pub fn read_instructions() -> String {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let manifest_path = Path::new(&manifest_dir).join("Cargo.toml");
    let metadata = MetadataCommand::new()
        .manifest_path(&manifest_path)
        .exec()
        .expect("Failed to read cargo metadata");

    let target_dir: Utf8PathBuf = metadata.target_directory;
    let json_path = target_dir.join("instructions.json");

    fs::read_to_string(&json_path)
        .unwrap_or_else(|_| panic!("Failed to read instructions.json at {}", json_path))
}

pub fn declare_program(input: TokenStream) -> TokenStream {
    let instructions: Vec<InstructionFn> =
        serde_json::from_str(&read_instructions()).expect("Invalid instructions.json");

    for instruction in instructions.iter() {
        println!("{}, {}", instruction.name, instruction.args[0]);
    }

    let input_str = parse_macro_input!(input as LitStr).value();

    TokenStream::new()
}
