use nom::digit;
use nom::types::CompleteStr;

use crate::assembler::Token;

named!(register_parser<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            reg_num: digit >>
            (
                Token::Register{
                reg_num: reg_num.parse::<u8>().unwrap()
                }
            )
        )
    )
);

#[test]
fn test_register_parser() {
    let result = register_parser(CompleteStr("$0"));
    assert_eq!(result.is_ok(), true);
    let result = register_parser(CompleteStr("0"));
    assert_eq!(result.is_ok(), false);
    let result = register_parser(CompleteStr("$a"));
    assert_eq!(result.is_ok(), false);
    let result = register_parser(CompleteStr("$10"));
    assert_eq!(result.is_ok(), false);
}
