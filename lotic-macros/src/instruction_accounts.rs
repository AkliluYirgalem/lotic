use camino::Utf8PathBuf;
use cargo_metadata::MetadataCommand;
use proc_macro::TokenStream;
use serde::Deserialize;
use std::fs;

use std::path::Path;

#[derive(Deserialize, Debug)]
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

    let target_dir: Utf8PathBuf = metadata.target_directory.into();
    let json_path = target_dir.join("instructions.json");

    fs::read_to_string(&json_path)
        .unwrap_or_else(|_| panic!("Failed to read instructions.json at {}", json_path))
}

pub fn instruction_accounts(_input: TokenStream) -> TokenStream {
    let instructions: Vec<InstructionFn> =
        serde_json::from_str(&read_instructions()).expect("Invalid instructions.json");

    println!("{:?}", instructions);

    TokenStream::new()
}
