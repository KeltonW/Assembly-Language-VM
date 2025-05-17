use nom::digit;
use nom::types::CompleteStr;

use crate::assembler::Token;

// parser for integers, prefixed by '#'
named!(integer_parser<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            reg_num: digit >>
            (
                Token::IntegerOperand{value: reg_num.parse::<i32>().unwrap()}
            )
        )
    )
);

#[test]
fn test_parse_integer_operand() {
    // test valid integer operand
    let result = integer_parser(CompleteStr("#10"));
    assert_eq!(result.is_ok(), true);
    let (rest, value) = result.unwrap();
    assert_eq!(rest, CompleteStr(""));
    assert_eq!(value, Token::IntegerOperand { value: 10 });

    // test invalid operand (no prefix)
    let result = integer_parser(CompleteStr("10"));
    assert_eq!(result.is_ok(), false);
}
