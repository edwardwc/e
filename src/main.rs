/// This tool is passed in a number and shows the number in both endianness:
/// Little endian is where the least significant byte is stored at the highest address. When we
/// look at numbers on our computer, they are mostly displayed in this format 
/// However, big endian is the format where the most significant byte is stored at the highest
/// address. Packets on the wire are organized in big endian format (network byte order).
/// Ex: port 80 (default HTTP port) is actually port 20480 on the wire due to the endian change
/// Usage: `e <number type, ex u32> <number>`
fn main() {
    let mut args = std::env::args().skip(1);
    let format = args.next().expect("No format provided");
    let unparsed_number = args.next().expect("No number passed in");

    println!("{}, {}", format, unparsed_number);

    match format.as_str() {
        // unsigned integers
        "u8" => {
            let number = unparsed_number.parse::<u8>().expect("Number was not a u8");
            println!("Number in big endian: {}", u8::to_be(number));
            println!("Number in little endian: {}", u8::to_le(number));

        }

        "u16" => {
            let number = unparsed_number.parse::<u16>().expect("Number was not a u16");
            println!("Number in big endian: {}", u16::to_be(number));
            println!("Number in little endian: {}", u16::to_le(number));

        }


        "u32" => {
            let number = unparsed_number.parse::<u32>().expect("Number was not a u32");
            println!("Number in big endian: {}", u32::to_be(number));
            println!("Number in little endian: {}", u32::to_le(number));

        }

        "u64" => {
            let number = unparsed_number.parse::<u64>().expect("Number was not a u64");
            println!("Number in big endian: {}", u64::to_be(number));
            println!("Number in little endian: {}", u64::to_le(number));

        }

        "u128" => {
            let number = unparsed_number.parse::<u128>().expect("Number was not a u128");
            println!("Number in big endian: {}", u128::to_be(number));
            println!("Number in little endian: {}", u128::to_le(number));

        }

        // signed integers

        "i8" => {
            let number = unparsed_number.parse::<i8>().expect("Number was not a i8");
            println!("Number in big endian: {}", i8::to_be(number));
            println!("Number in little endian: {}", i8::to_le(number));

        }

        "i16" => {
            let number = unparsed_number.parse::<i16>().expect("Number was not a i16");
            println!("Number in big endian: {}", i16::to_be(number));
            println!("Number in little endian: {}", i16::to_le(number));

        }

        "i32" => {
            let number = unparsed_number.parse::<i32>().expect("Number was not a i32");
            println!("Number in big endian: {}", i32::to_be(number));
            println!("Number in little endian: {}", i32::to_le(number));

        }

        "i64" => {
            let number = unparsed_number.parse::<i64>().expect("Number was not a i64");
            println!("Number in big endian: {}", i64::to_be(number));
            println!("Number in little endian: {}", i64::to_le(number));

        }

        "i128" => {
            let number = unparsed_number.parse::<i128>().expect("Number was not a i128");
            println!("Number in big endian: {}", i128::to_be(number));
            println!("Number in little endian: {}", i128::to_le(number));

        }

        _ => panic!("Unknown format {}", format)
    }    
}
