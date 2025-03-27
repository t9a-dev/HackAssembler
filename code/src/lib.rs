use anyhow::Result;

const D_REGISTER_TOKEN: char = 'D';
const A_REGISTER_TOKEN: char = 'A';
const M_REGISTER_TOKEN: char = 'M';
const PLUS_TOKEN: char = '+';
const MINUS_TOKEN: char = '-';
const NOT_TOKEN: char = '!';
const OR_TOKEN: char = '|';

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
        //0,1,-1は入力を利用していないので固定で返す。
        match v {
            "0" => return Ok("0101010".to_string()),
            "1" => return Ok("0111111".to_string()),
            "-1" => return Ok("0111010".to_string()),
            _ => (),
        }

        let mut result = vec![0; 7];
        let increment_token = format!("{}{}", PLUS_TOKEN, "1");
        let minus_dregister_token = format!("{}{}", MINUS_TOKEN, D_REGISTER_TOKEN);
        let minus_aregister_token = format!("{}{}", MINUS_TOKEN, A_REGISTER_TOKEN);
        let minus_mregister_token = format!("{}{}", MINUS_TOKEN, M_REGISTER_TOKEN);
        let or_aregister_token = format!("{}{}", OR_TOKEN, A_REGISTER_TOKEN);
        let or_mregister_token = format!("{}{}", OR_TOKEN, M_REGISTER_TOKEN);
        let alu_x_include_tokens = vec![D_REGISTER_TOKEN];
        let alu_nx_include_tokens = vec![
            increment_token.to_string(),
            minus_aregister_token.clone(),
            minus_mregister_token.clone(),
            OR_TOKEN.to_string(),
        ];
        let alu_y_include_tokens = vec![A_REGISTER_TOKEN, M_REGISTER_TOKEN];
        let alu_ny_include_tokens = vec![
            increment_token.to_string(),
            minus_dregister_token.clone(),
            OR_TOKEN.to_string(),
        ];
        let alu_f_bit_tokens = vec![MINUS_TOKEN, PLUS_TOKEN];
        let alu_no_bit_tokens = vec![
            NOT_TOKEN.to_string(),
            minus_dregister_token,
            minus_aregister_token,
            minus_mregister_token,
            increment_token.to_string(),
            or_aregister_token,
            or_mregister_token,
        ];

        // comp a bit
        result[0] = v.contains(M_REGISTER_TOKEN) as u8;
        // ALU zx bit
        result[1] = !v.chars().any(|c| alu_x_include_tokens.contains(&c)) as u8;
        // ALU nx bit
        result[2] = alu_nx_include_tokens
            .iter()
            .any(|token| !v.contains(D_REGISTER_TOKEN) || v.contains(token))
            as u8;
        // ALU zy bit
        result[3] = !v.chars().any(|c| alu_y_include_tokens.contains(&c)) as u8;
        // ALU ny bit
        result[4] = alu_ny_include_tokens.iter().any(|token| {
            (!v.contains(A_REGISTER_TOKEN) && !v.contains(M_REGISTER_TOKEN)) || v.contains(token)
        }) as u8;
        // ALU f bit
        result[5] = v.chars().any(|c| alu_f_bit_tokens.contains(&c)) as u8;
        // ALU no bit
        result[6] = alu_no_bit_tokens
            .iter()
            .any(|token| v.starts_with(token) || v.contains(token)) as u8;

        Ok(result.into_iter().map(|b| (b'0' + b) as char).collect())
    }
    pub fn jump(v: &str) -> Result<String> {
        match v {
            "JGT" => Ok("001".to_string()),
            "JEQ" => Ok("010".to_string()),
            "JGE" => Ok("011".to_string()),
            "JLT" => Ok("100".to_string()),
            "JNE" => Ok("101".to_string()),
            "JLE" => Ok("110".to_string()),
            "JMP" => Ok("111".to_string()),
            _ => Ok("000".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Code;

    #[test]
    fn playground() -> Result<()> {
        let binary_string: String = vec![0, 1, 0, 1, 1, 1]
            .into_iter()
            .map(|b| (b'0' + b) as char)
            .collect();
        assert_eq!(binary_string, "010111".to_string());
        Ok(())
    }

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
        //a==0
        assert_eq!(Code::comp("0")?, "0101010");
        assert_eq!(Code::comp("1")?, "0111111");
        assert_eq!(Code::comp("-1")?, "0111010");
        assert_eq!(Code::comp("D")?, "0001100");
        assert_eq!(Code::comp("A")?, "0110000");
        assert_eq!(Code::comp("!D")?, "0001101");
        assert_eq!(Code::comp("!A")?, "0110001");
        assert_eq!(Code::comp("-D")?, "0001111");
        assert_eq!(Code::comp("-A")?, "0110011");
        assert_eq!(Code::comp("D+1")?, "0011111");
        assert_eq!(Code::comp("A+1")?, "0110111");
        assert_eq!(Code::comp("D-1")?, "0001110");
        assert_eq!(Code::comp("A-1")?, "0110010");
        assert_eq!(Code::comp("D+A")?, "0000010");
        assert_eq!(Code::comp("D-A")?, "0010011");
        assert_eq!(Code::comp("A-D")?, "0000111");
        assert_eq!(Code::comp("D&A")?, "0000000");
        assert_eq!(Code::comp("D|A")?, "0010101");

        //a==1
        assert_eq!(Code::comp("M")?, "1110000");
        assert_eq!(Code::comp("!M")?, "1110001");
        assert_eq!(Code::comp("-M")?, "1110011");
        assert_eq!(Code::comp("M+1")?, "1110111");
        assert_eq!(Code::comp("M-1")?, "1110010");
        assert_eq!(Code::comp("D+M")?, "1000010");
        assert_eq!(Code::comp("D-M")?, "1010011");
        assert_eq!(Code::comp("M-D")?, "1000111");
        assert_eq!(Code::comp("D&M")?, "1000000");
        assert_eq!(Code::comp("D|M")?, "1010101");
        Ok(())
    }

    #[test]
    fn test_jump() -> Result<()> {
        assert_eq!(Code::jump("null")?, "000");
        assert_eq!(Code::jump("JGT")?, "001");
        assert_eq!(Code::jump("JEQ")?, "010");
        assert_eq!(Code::jump("JGE")?, "011");
        assert_eq!(Code::jump("JLT")?, "100");
        assert_eq!(Code::jump("JNE")?, "101");
        assert_eq!(Code::jump("JLE")?, "110");
        assert_eq!(Code::jump("JMP")?, "111");

        Ok(())
    }
}
