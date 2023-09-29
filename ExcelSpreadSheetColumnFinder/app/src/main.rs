fn main() {
    println!("{}", convert_to_title(18767));
}

pub fn convert_to_title(column_number: i32) -> String {
    // finds quotient and remainder
    let mut column_number = column_number;
    let mut result = String::new();

    // process repeats till quotient is 0
    while column_number > 0 {
        // finds remainder of column number
        let remainder = (column_number - 1) % 26;
        // adds the remainder to the ASCII val of 'A' to get the letter of the remainder
        let letter = (remainder as u8 + 65) as char;
        // the letter is then pushed to the front of the result
        result.insert(0, letter);
        // the column number is then updated to be the quotient
        column_number = (column_number - 1) / 26;
    }

    result
}
// method that works for up to 2 letters:
/*
let remainder: u8 = ((&column_number % 26) + 64) as u8;
let quotient: u8 = ((&column_number / 26) + 64) as u8;
if quotient == 64 as u8 {
    return String::from(((column_number + 64) as u8) as char);
}
return format!("{}{}", quotient as char, remainder as char);
*/

//A = 1
//AA = 27
//ZY = 701
//"FXSHRXW" = 2147483647
