use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::types::CompleteStr;

named!(opcode_load<CompleteStr, Token>,
    do_parse!(
        tag!("load") >> (Token::Op{code:Opcode::LOAD})
    )
);

mod tests {
    use super::*;

    #[test]
    fn opcode_load_test() {
        // test that opcode is detected and parsed correctly
        let result = opcode_load(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(token, Token::Op { code: Opcode::LOAD });

        // test that invalid opcode is not detected
        let result = opcode_load(CompleteStr("aold"));
        assert_eq!(result.is_ok(), false);
    }
}
