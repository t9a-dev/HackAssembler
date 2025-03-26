use anyhow::Result;

pub struct Code {}

impl Code {
    pub fn dest(v: u8) -> Result<String> {
        todo!()
    }
    pub fn comp(v: u8) -> Result<String> {
        todo!()
    }
    pub fn jump(v: u8) -> Result<String> {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Code;

    #[test]
    fn it_works() -> Result<()> {
        let v = 10;
        let _dest = Code::dest(v);
        let _comp = Code::comp(v);
        let _jump = Code::jump(v);
        Ok(())
    }
}
