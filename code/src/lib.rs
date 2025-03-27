use anyhow::Result;

pub struct Code {}

impl Code {
    pub fn dest(v: &str) -> Result<String> {
        Ok(format!(
            "{}{}{}",
            v.contains('A') as u8,
            v.contains('D') as u8,
            v.contains('M') as u8
        )
        .to_string())
    }
    pub fn comp(v: &str) -> Result<String> {
        todo!()
    }
    pub fn jump(v: &str) -> Result<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Code;

    #[test]
    fn test_dest() -> Result<()> {
        assert_eq!(Code::dest("null")?, "000");
        assert_eq!(Code::dest("M")?, "001");
        assert_eq!(Code::dest("D")?, "010");
        assert_eq!(Code::dest("DM")?, "011");
        assert_eq!(Code::dest("A")?, "100");
        assert_eq!(Code::dest("AM")?, "101");
        assert_eq!(Code::dest("AD")?, "110");
        assert_eq!(Code::dest("ADM")?, "111");
        Ok(())
    }

    #[test]
    fn test_comp() -> Result<()> {
        assert_eq!(Code::comp("A+1")?, "0110111");
        Ok(())
    }

    #[test]
    fn test_jump() -> Result<()> {
        assert_eq!(Code::jump("JNE")?, "101");
        Ok(())
    }
}
