use anyhow::Result;
use clap::Parser;
use parser::InstructionType;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use symbol_table::SymbolTable;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arg {
    // HackAsembler File Path
    #[arg(value_name = "FILE_NAME.asm", short)]
    file: String,
}

fn main() -> Result<()> {
    if let Err(e) = hack_assembler(&Arg::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    Ok(())
}

fn hack_assembler(config: &Arg) -> Result<String> {
    let asm_file = Path::new(config.file.as_str());
    let mut symbol_table = SymbolTable::new();
    first_pass(asm_file.to_string_lossy().to_string(), &mut symbol_table)?;
    let hack_file_path = second_pass(asm_file.to_string_lossy().to_string(), &mut symbol_table)?;

    println!("Assembled: {}", &hack_file_path);

    Ok(hack_file_path)
}

fn first_pass(asm_file_path: String, symbol_table: &mut SymbolTable) -> Result<()> {
    let mut asm_parser = parser::Parser::new(asm_file_path.as_str());
    let mut row_number: u16 = 0;

    while asm_parser.has_more_lines()? {
        asm_parser.advance()?;
        match asm_parser.instruction_type()? {
            Some(InstructionType::L) => {
                symbol_table.add_entry(asm_parser.symbol()?.unwrap().as_str(), row_number)?;
            }
            Some(InstructionType::A) | Some(InstructionType::C) => {
                row_number += 1;
            }
            None => (),
        }

        if asm_parser.has_more_lines()? == false {
            break;
        }
    }

    Ok(())
}

fn second_pass(asm_file_path: String, symbol_table: &mut SymbolTable) -> Result<String> {
    let asm_file = Path::new(asm_file_path.as_str());
    let file_name = asm_file
        .file_stem()
        .expect("get file_stem error")
        .to_string_lossy();
    let hack_file_path = asm_file
        .parent()
        .expect("get hack save dir failed.")
        .join(format!("{}.{}", file_name, "hack"));
    let mut hack_file = File::create(&hack_file_path).unwrap();
    let mut asm_parser = parser::Parser::new(&asm_file_path.as_str());
    let mut variable_ram_address: u16 = 16;

    while asm_parser.has_more_lines()? {
        asm_parser.advance()?;
        match asm_parser.instruction_type()? {
            Some(parser::InstructionType::A) => {
                //文字列であれば変数として扱い、数値であればそのままバイナリに変換して書き込む
                let symbol = asm_parser.symbol()?.unwrap();
                match symbol.parse::<u16>() {
                    //数値に変換正羽したのでバイナリに変換して書き込む
                    Ok(numeric_value) => {
                        let binary_string = format_16bit_binary_string(numeric_value);
                        hack_file.write(binary_string.as_bytes())?;
                    }
                    //数値に変換できなかったので変数として扱う
                    Err(_) => {
                        if symbol_table.contains(&symbol)? {
                            let address = symbol_table.get_address(symbol.as_str())?;
                            hack_file.write(format_16bit_binary_string(address).as_bytes())?;
                        } else {
                            symbol_table.add_entry(symbol.as_str(), variable_ram_address)?;
                            hack_file.write(
                                format_16bit_binary_string(variable_ram_address).as_bytes(),
                            )?;
                            variable_ram_address += 1;
                        }
                    }
                }
            }
            Some(parser::InstructionType::C) => {
                let dest_binary_string = code::Code::dest(asm_parser.dest()?)?;
                let comp_binary_string = code::Code::comp(asm_parser.comp()?)?;
                let jump_binary_string = code::Code::jump(asm_parser.jump()?)?;
                let c_instruction_binary_string = format!(
                    "111{}{}{}\n",
                    comp_binary_string, dest_binary_string, jump_binary_string
                );
                hack_file.write(c_instruction_binary_string.as_bytes())?;
            }
            Some(parser::InstructionType::L) => (),
            None => (),
        }

        if asm_parser.has_more_lines()? == false {
            break;
        }
    }

    Ok(hack_file_path.to_string_lossy().to_string())
}

fn format_16bit_binary_string(v: u16) -> String {
    format!("{:016b}\n", v)
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
        let hack_file_path = hack_assembler(&config)?;
        let mut hack_file = File::open(hack_file_path)?;
        let mut buffer = String::new();
        let _ = hack_file.read_to_string(&mut buffer);
        assert_eq!(buffer, "0000000000101000\n1110110111011101\n");

        let _ = fs::remove_file(test_file);

        Ok(())
    }

    #[test]
    fn test_first_pass() -> Result<()> {
        let test_file_path = create_test_file("(START)\n@40\n(LOOP)\nDM=A+1;JNE\n(STOP)\n(END)\n");
        let mut symbol_table = SymbolTable::new();
        first_pass(test_file_path.clone(), &mut symbol_table)?;

        assert_eq!(symbol_table.get_address("START")?, 0);
        assert_eq!(symbol_table.get_address("LOOP")?, 1);
        assert_eq!(symbol_table.get_address("STOP")?, 2);
        assert_eq!(symbol_table.get_address("END")?, 2);

        let _ = fs::remove_file(test_file_path);
        Ok(())
    }

    #[test]
    fn test_second_pass() -> Result<()> {
        let test_file_path = create_test_file("(START)\n@40\n(LOOP)\nDM=A+1;JNE\n(STOP)\nD=A\n(END)\n");
        let mut symbol_table = SymbolTable::new();
        first_pass(test_file_path.clone(), &mut symbol_table)?;
        second_pass(test_file_path.clone(), &mut symbol_table)?;

        assert_eq!(symbol_table.get_address("START")?, 0);
        assert_eq!(symbol_table.get_address("LOOP")?, 1);
        assert_eq!(symbol_table.get_address("STOP")?, 2);
        assert_eq!(symbol_table.get_address("END")?, 3);

        let _ = fs::remove_file(test_file_path);
        Ok(())
    }
}
