use anyhow::Result;
use std::{fs::File, io::{BufRead, BufReader,Read} };

pub enum InstructionType {
   A,
   C,
   L, 
}
pub struct Parser{
    assembly: Box<dyn BufRead>,
    current_instruction: String,
}

impl Parser{
    pub fn new(filename: &str)-> Self{
        Self{
            assembly: Box::new(BufReader::new(File::open(filename).unwrap())),
            current_instruction: String::new(),
        }
    }

    fn has_more_lines(&mut self) -> Result<bool>{
        Ok(self.assembly.fill_buf()?.iter().next().is_some())
    }

    fn advance(&mut self) -> Result<()>{
        if self.has_more_lines()?{
           // //で始まるコメント行と空白を無視して次の行を読み込む"
           self.current_instruction = self.assembly.as_mut().lines().next().unwrap()?;
        }

        Ok(())
    }

    fn instruction_type()->Result<InstructionType>{
        todo!()
    }

    fn symbol() -> Result<String>{
        todo!()
    }

    fn dest() -> Result<String>{
        todo!()
    }

    fn comp() -> Result<String>{
        todo!()
    }

    fn jump() -> Result<String>{
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Write, path::Path};

    use super::*;
    use rand::distr::{Alphanumeric,SampleString};

    fn create_test_file(file_content: &str) -> String{
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
        parser.assembly
        .lines()
        .into_iter()
        .for_each(|line| println!("{}",line.unwrap()));
        
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_has_more_lines()-> Result<()>{
        let file_content = "line1";
        let test_file = create_test_file(&file_content);

        let mut parser = Parser::new(&test_file);
        let _ = fs::remove_file(test_file);

        assert_eq!(parser.has_more_lines()?,true);
        //次の命令を読み込む
        parser.advance()?;
        assert_eq!(parser.current_instruction,file_content);
        //ファイルには1行しか書き込んでおらず次の行は存在しない。
        assert_eq!(parser.has_more_lines()?,false);

        Ok(())
    }

    #[test]
    fn test_advance()-> Result<()>{
        let file_content = "line1\n//this comment\n \nline2";
        let test_file = create_test_file(&file_content);
        let mut parser = Parser::new(&test_file);
        let _ = fs::remove_file(test_file);

        //次の命令を読み込む
        parser.advance()?;
        assert_eq!(parser.current_instruction,"line1");

        //コメント行を無視して"line2"が読み込まれている。
        parser.advance()?;
        assert_eq!(parser.current_instruction,"line2");

        //空白行を除くと残りの行は存在しない
        assert_eq!(parser.has_more_lines()?,false);

        //コメント行を除くと残りの行は存在しない
        assert_eq!(parser.has_more_lines()?,false);

        Ok(())
    }
}
