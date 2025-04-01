use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arg {
    // HackAsembler File Path
    #[arg(value_name = "FILE_NAME.asm", short)]
    file: String,
}

fn main() -> Result<()> {
    if let Err(e) = hack_assembler(Arg::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    Ok(())
}

fn hack_assembler(config: Arg) -> Result<PathBuf> {
    println!("I am HackAssembler!");
    let asm_file = Path::new(&config.file);
    let file_name = asm_file
        .file_stem()
        .expect("get file_stem error")
        .to_string_lossy();
    let hack_file_path = asm_file
        .parent()
        .expect("get hack save dir failed.")
        .join(format!("{}.{}", file_name, "hack"));
    let mut hack_file = File::create(&hack_file_path).unwrap();
    let mut asm_parser = parser::Parser::new(&config.file);

    while asm_parser.has_more_lines()? {
        asm_parser.advance()?;
        match asm_parser.instruction_type()? {
            Some(parser::InstructionType::A) => {
                let binary_string =
                    symbol_to_16bit_binary(asm_parser.symbol()?.expect("symbol empty."))?;
                let _ = hack_file.write(binary_string.as_bytes());
            }
            Some(parser::InstructionType::C) => {
                let dest_binary_string = code::Code::dest(asm_parser.dest()?)?;
                let comp_binary_string = code::Code::comp(asm_parser.comp()?)?;
                let jump_binary_string = code::Code::jump(asm_parser.jump()?)?;
                let c_instruction_binary_string = format!(
                    "111{}{}{}\n",
                    comp_binary_string, dest_binary_string, jump_binary_string
                );
                let _ = hack_file.write(c_instruction_binary_string.as_bytes());
            }
            Some(parser::InstructionType::L) => {
                todo!()
            }
            None => (),
        }

        if asm_parser.has_more_lines()? == false {
            break;
        }
    }

    Ok(hack_file_path)
}

fn symbol_to_16bit_binary(symbol: String) -> Result<String> {
    let symbol: u8 = symbol.parse().expect("symbol to u8 parse error.");

    Ok(format!("{:016b}\n", symbol))
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::{Read, Write},
        path::Path,
    };

    use super::*;
    use rand::distr::{Alphanumeric, SampleString};

    fn create_test_file(file_content: &str) -> String {
        let filename = Alphanumeric.sample_string(&mut rand::rng(), 5);
        //bacon testでファイル変更検知が発生しないようにtargetディレクトリにテストファイルを作成する。
        let _ = fs::create_dir_all("../target/test/data");
        let file_path = Path::new("../target/test/data").join(&filename);
        let mut file = File::create(&file_path).unwrap();
        file.write(file_content.as_bytes()).unwrap();

        file_path.to_string_lossy().to_string()
    }

    #[test]
    fn playground() {
        let path = Path::new("/a/b/c.txt");
        assert_eq!(path.parent(), Some(Path::new("/a/b/")));
    }

    #[test]
    fn test_hack_assemble() -> Result<()> {
        let test_file = create_test_file("@40\nDM=A+1;JNE");
        let config = Arg {
            file: test_file.clone(),
        };
        let hack_file_path = hack_assembler(config)?;
        let mut hack_file = File::open(hack_file_path)?;
        let mut buffer = String::new();
        let _ = hack_file.read_to_string(&mut buffer);
        assert_eq!(buffer, "0000000000101000\n1110110111011101\n");

        let _ = fs::remove_file(test_file);

        Ok(())
    }
}
