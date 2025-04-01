use anyhow::Result;
use std::collections::HashMap;

pub struct SymbolTable {
    pub entries: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> Self {
        //　定義済みのシンボルとアドレスを登録
        let mut entries = HashMap::new();
        entries.insert("R0".to_string(), 0);
        entries.insert("R1".to_string(), 1);
        entries.insert("R2".to_string(), 2);
        entries.insert("R3".to_string(), 3);
        entries.insert("R4".to_string(), 4);
        entries.insert("R5".to_string(), 5);
        entries.insert("R6".to_string(), 6);
        entries.insert("R7".to_string(), 7);
        entries.insert("R8".to_string(), 8);
        entries.insert("R9".to_string(), 9);
        entries.insert("R10".to_string(), 10);
        entries.insert("R11".to_string(), 11);
        entries.insert("R12".to_string(), 12);
        entries.insert("R13".to_string(), 13);
        entries.insert("R14".to_string(), 14);
        entries.insert("R15".to_string(), 15);

        entries.insert("SP".to_string(), 0);
        entries.insert("LCL".to_string(), 1);
        entries.insert("ARG".to_string(), 2);
        entries.insert("THIS".to_string(), 3);
        entries.insert("THAT".to_string(), 4);
        entries.insert("SCREEN".to_string(), 16384);
        entries.insert("KBD".to_string(), 24576);

        Self { entries }
    }

    pub fn add_entry(&mut self, symbol: &str, address: u16) -> Result<()> {
        self.entries.insert(symbol.to_string(), address);
        Ok(())
    }

    pub fn contains(&self, symbol: &str) -> Result<bool> {
        Ok(self.entries.contains_key(symbol))
    }

    pub fn get_address(&self, symbol: &str) -> Result<u16> {
        Ok(*self.entries.get(symbol).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let mut table = SymbolTable::new();
        table.add_entry("sum", 5)?;
        assert_eq!(table.entries.contains_key("sum"), true);
        assert_eq!(table.get_address("sum")?, 5);
        assert_eq!(table.contains("empty")?, false);
        Ok(())
    }
}
