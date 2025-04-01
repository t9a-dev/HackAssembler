use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const COMMENT_OUT_TOKEN: &str = "//";
const A_INSTRUCTION_TOKEN: char = '@';
const C_INSTRUCTION_TOKEN_EQUAL: char = '=';
const C_INSTRUCTION_TOKEN_SEMICOLON: char = ';';
const L_INSTRUCTION_TOKEN_START: char = '(';
const L_INSTRUCTION_TOKEN_END: char = ')';

#[derive(Debug, PartialEq)]
pub enum InstructionType {
    A,
    C,
    L,
}
pub struct Parser {
    assembly: Box<dyn BufRead>,
    current_instruction: Option<String>,
}

impl Parser {
    pub fn new(filename: &str) -> Self {
        Self {
            assembly: Box::new(BufReader::new(File::open(filename).unwrap())),
            current_instruction: None,
        }
    }

    pub fn has_more_lines(&mut self) -> Result<bool> {
        Ok(self.assembly.fill_buf()?.iter().next().is_some())
    }

    pub fn advance(&mut self) -> Result<()> {
        // //で始まるコメント行と空白を無視して次の行を読み込む
        while self.has_more_lines()? {
            self.current_instruction = match self.assembly.as_mut().lines().next().unwrap() {
                Ok(line) if line.chars().all(char::is_whitespace) => None, //空白の場合は無視
                Ok(line) if line.starts_with(COMMENT_OUT_TOKEN) => None,   //コメント行の場合は無視
                Ok(line) => Some(line),
                Err(_) => None,
            };
            if self.current_instruction.is_some() {
                break;
            }
        }
        Ok(())
    }

    pub fn instruction_type(&self) -> Result<Option<InstructionType>> {
        match &self.current_instruction {
            Some(instruction) if instruction.starts_with(A_INSTRUCTION_TOKEN) => {
                Ok(Some(InstructionType::A))
            }
            Some(instruction)
                if instruction.starts_with(L_INSTRUCTION_TOKEN_START)
                    && instruction.ends_with(L_INSTRUCTION_TOKEN_END) =>
            {
                Ok(Some(InstructionType::L))
            }
            Some(instruction)
                if instruction.contains(C_INSTRUCTION_TOKEN_SEMICOLON)
                    || instruction.contains(C_INSTRUCTION_TOKEN_EQUAL) =>
            {
                Ok(Some(InstructionType::C))
            }
            Some(instruction) => panic!(
                "parse instruction type error. instruction_value: {:?}",
                instruction
            ),
            None => Ok(None),
        }
    }

    pub fn symbol(&self) -> Result<Option<String>> {
        match self.instruction_type()?.unwrap() {
            InstructionType::A => Ok(Some(
                self.current_instruction
                    .clone()
                    .unwrap()
                    .chars()
                    .filter(|c| *c != A_INSTRUCTION_TOKEN)
                    .collect(),
            )),
            InstructionType::L => Ok(Some(
                self.current_instruction
                    .clone()
                    .unwrap()
                    .chars()
                    .filter(|c| *c != L_INSTRUCTION_TOKEN_START && *c != L_INSTRUCTION_TOKEN_END)
                    .collect(),
            )),
            _ => Ok(None),
        }
    }

    pub fn dest(&self) -> Result<&str> {
        if self.instruction_type()?.unwrap() != InstructionType::C {
            return Ok("");
        }
        match &self.current_instruction {
            Some(instruction) if instruction.contains(C_INSTRUCTION_TOKEN_EQUAL) => Ok(instruction
                .split(C_INSTRUCTION_TOKEN_EQUAL)
                .into_iter()
                .nth(0)
                .unwrap()),
            Some(_) => Ok(""),
            None => Ok(""),
        }
    }

    pub fn comp(&self) -> Result<&str> {
        if self.instruction_type()?.unwrap() != InstructionType::C {
            return Ok("");
        }
        match &self.current_instruction {
            Some(instruction) => {
                if instruction.contains(C_INSTRUCTION_TOKEN_EQUAL) {
                    Ok(instruction
                        .split(&[C_INSTRUCTION_TOKEN_EQUAL, C_INSTRUCTION_TOKEN_SEMICOLON][..])
                        .into_iter()
                        .nth(1)
                        .unwrap())
                } else {
                    Ok(instruction
                        .split(C_INSTRUCTION_TOKEN_SEMICOLON)
                        .into_iter()
                        .nth(0)
                        .unwrap())
                }
            }
            None => Ok(""),
        }
    }

    pub fn jump(&self) -> Result<&str> {
        if self.instruction_type()?.unwrap() != InstructionType::C {
            return Ok("");
        }
        match &self.current_instruction {
            Some(instruction) if instruction.contains(C_INSTRUCTION_TOKEN_SEMICOLON) => {
                if instruction.contains(C_INSTRUCTION_TOKEN_EQUAL) {
                    Ok(instruction
                        .split(&[C_INSTRUCTION_TOKEN_EQUAL, C_INSTRUCTION_TOKEN_SEMICOLON][..])
                        .into_iter()
                        .nth(2)
                        .unwrap())
                } else {
                    Ok(instruction
                        .split(C_INSTRUCTION_TOKEN_SEMICOLON)
                        .into_iter()
                        .nth(1)
                        .unwrap())
                }
            }
            Some(_) => Ok(""),
            None => Ok(""),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Write, path::Path};

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
    fn test_constructor() {
        let test_file = create_test_file("");
        let parser = Parser::new(&test_file);
        parser
            .assembly
            .lines()
            .into_iter()
            .for_each(|line| println!("{}", line.unwrap()));

        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_has_more_lines() -> Result<()> {
        let file_content = "@123\n//this comment\n \n(START)\nD;JGT";
        let test_file = create_test_file(&file_content);

        let mut parser = Parser::new(&test_file);
        let _ = fs::remove_file(test_file);

        //@123
        parser.advance()?;
        assert_eq!(parser.has_more_lines()?, true);

        //(START)
        parser.advance()?;
        assert_eq!(parser.has_more_lines()?, true);

        //D;JGT
        parser.advance()?;
        assert_eq!(parser.has_more_lines()?, false);

        Ok(())
    }

    #[test]
    fn test_advance() -> Result<()> {
        let file_content = "@123\n//this comment\n \n(START)\nD;JGT";
        let test_file = create_test_file(&file_content);
        let mut parser = Parser::new(&test_file);
        let _ = fs::remove_file(test_file);

        //次の命令を読み込む
        parser.advance()?;
        assert_eq!(parser.current_instruction.clone().unwrap(), "@123");

        //コメント行を無視して"line2"が読み込まれている。
        parser.advance()?;
        assert_eq!(parser.current_instruction.clone().unwrap(), "(START)");

        //空白行を除くと残りの行は存在しない
        parser.advance()?;
        assert_eq!(parser.current_instruction.clone().unwrap(), "D;JGT");

        //コメント行を除くと残りの行は存在しない
        assert_eq!(parser.has_more_lines()?, false);

        Ok(())
    }

    #[test]
    fn test_instruction() -> Result<()> {
        let file_content = "@123\n//this comment\n \n(START)\nD;JGT";
        let test_file = create_test_file(&file_content);
        let mut parser = Parser::new(&test_file);
        let _ = fs::remove_file(test_file);

        //次の命令を読み込む
        parser.advance()?;
        assert_eq!(parser.instruction_type()?.unwrap(), InstructionType::A);

        //次の命令を読み込む
        parser.advance()?;
        assert_eq!(parser.instruction_type()?.unwrap(), InstructionType::L);

        //次の命令を読み込む
        parser.advance()?;
        assert_eq!(parser.instruction_type()?.unwrap(), InstructionType::C);

        //次の行はなくadvance()を実行しても現在の命令は変わらない
        parser.advance()?;
        assert_eq!(parser.instruction_type()?.unwrap(), InstructionType::C);

        Ok(())
    }

    #[test]
    fn test_symbol() -> Result<()> {
        let file_content = "@123\n//this comment\n \n(START)\n@sum\nD;JGT";
        let test_file = create_test_file(&file_content);
        let mut parser = Parser::new(&test_file);
        let _ = fs::remove_file(test_file);

        //@123
        parser.advance()?;
        assert_eq!(parser.symbol()?.unwrap(), "123");

        //(START)
        parser.advance()?;
        assert_eq!(parser.symbol()?.unwrap(), "START");

        //sum
        parser.advance()?;
        assert_eq!(parser.symbol()?.unwrap(), "sum");

        //D;JGT
        parser.advance()?;
        assert_eq!(parser.symbol()?, None);

        Ok(())
    }

    #[test]
    fn test_dest_comp_jump() -> Result<()> {
        let file_content = "D=D+1;JLE\nDM=D|A;JLT\nD&A;JMP\nD=A";
        let test_file = create_test_file(&file_content);
        let mut parser = Parser::new(&test_file);
        let _ = fs::remove_file(test_file);

        parser.advance()?;
        assert_eq!(parser.dest()?, "D");
        assert_eq!(parser.comp()?, "D+1");
        assert_eq!(parser.jump()?, "JLE");

        parser.advance()?;
        assert_eq!(parser.dest()?, "DM");
        assert_eq!(parser.comp()?, "D|A");
        assert_eq!(parser.jump()?, "JLT");

        parser.advance()?;
        assert_eq!(parser.dest()?, "");
        assert_eq!(parser.comp()?, "D&A");
        assert_eq!(parser.jump()?, "JMP");

        parser.advance()?;
        assert_eq!(parser.dest()?, "D");
        assert_eq!(parser.comp()?, "A");
        assert_eq!(parser.jump()?, "");
        Ok(())
    }
}
