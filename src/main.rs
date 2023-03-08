fn convert_message(message: &str) {
    // 16進数に変換する
    let hex_string = message
        .chars()
        .map(|c| format!("{:02x}", c as u32))
        .collect::<String>();

    println!("hex        : {}", hex_string);

    // 2進数に変換する
    let binary_string = message
        .chars()
        .map(|c| format!("{:08b}", c as u32))
        .collect::<String>();

    println!("bin        : {}", binary_string);

    // Vec<u8>
    let message_bytes = message.as_bytes().to_vec();

    println!("bec_u8     : {:?}", message_bytes);

    binary_to_string(&binary_string);
    hex_to_string(&hex_string);
}

fn convert_utf8_message(message: &str) {
    // UTF-8文字列をバイト列に変換
    let bytes = message.as_bytes();

    // バイト列を16進数表記に変換
    let hex_str = bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<String>();
    println!("utf8 hex   : {}", hex_str);

    // バイト列を2進数表記に変換
    let binary_str = bytes
        .iter()
        .map(|b| format!("{:08b}", b))
        .collect::<String>();
    println!("utf8 bin   : {}", binary_str);

    // バイト列をVec<u8>に変換
    let vec_bytes = bytes.to_vec();
    println!("utf8 vec_u8: {:?}", vec_bytes);

    binary_to_utf8_string(&binary_str);
    hex_to_utf8_string(&hex_str);
}

fn convert_shiftjis_message(message: &str) {
    // Shift-JIS文字列をバイト列に変換
    let (bytes, _, _) = encoding_rs::SHIFT_JIS.encode(message);

    // バイト列を16進数表記に変換
    let hex_str = bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<String>();
    println!("shift_jis hex : {}", hex_str);

    // バイト列を2進数表記に変換
    let binary_str = bytes
        .iter()
        .map(|b| format!("{:08b}", b))
        .collect::<String>();
    println!("shift_jis bin : {}", binary_str);

    // バイト列をVec<u8>に変換
    let vec_bytes = bytes.to_vec();
    println!("shift_jis vec_u8 : {:?}", vec_bytes);

    // binary_to_string(&binary_str);
    // hex_to_string(&hex_str);
}

fn binary_to_string(binary_str: &str) {
    let result = binary_str
        .chars()
        .collect::<Vec<_>>()
        .chunks(8)
        .map(|chunk| {
            let byte = chunk.iter().collect::<String>();
            char::from_u32(u32::from_str_radix(&byte, 2).unwrap()).unwrap()
        })
        .collect::<String>();

    println!("{}", result);
}

fn hex_to_string(hex_str: &str) {
    let result =  hex_str
    .chars()
    .collect::<Vec<_>>()
    .chunks(2)
    .map(|chunk| {
        let byte = chunk.iter().collect::<String>();
        char::from_u32(u32::from_str_radix(&byte, 16).unwrap()).unwrap()
    })
    .collect::<String>();

    println!("{}", result);
}

fn binary_to_utf8_string(binary_str: &str) {
    let mut bytes = Vec::new();
    for i in 0..(binary_str.len() / 8) {
        let byte_str = &binary_str[i * 8..(i + 1) * 8];
        let byte = u8::from_str_radix(byte_str, 2).unwrap();
        bytes.push(byte);
    }

    let result = String::from_utf8(bytes).unwrap();

    println!("{}", result);
}

fn hex_to_utf8_string(hex_str: &str) {
    let bytes = hex::decode(hex_str).unwrap();

    let result = String::from_utf8(bytes).unwrap();

    println!("{}", result);
}

fn main() {
    // let message = "こんにちは";
    // let message = "こんにちは\nOneMans'sです。";
    let message = "hello world";
    convert_message(message);
    convert_utf8_message(message);
    convert_shiftjis_message(message);
}
