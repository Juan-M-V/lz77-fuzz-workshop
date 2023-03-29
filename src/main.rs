use std::collections::HashMap;
const WINDOW_SIZE: usize = 256;

fn main() {
    let message = "hello sun, hello cat";
    let bytes = message.as_bytes();
    let encoded = lz77_encode(bytes);
    /*for (i, c) in message.to_string().chars().enumerate() {
        print!("{} = {:?}; ", c, bytes[i]);
    }
    println!("");
    println!("{buf:?}");*/
    let decoded = lz77_decode(&encoded);
    assert_eq!(decoded, bytes);
}

fn lz77_encode(data: &[u8]) -> Vec<u8> {
    let mut index = 0;
    let mut move_window = 1;
    let mut dict: HashMap<&[u8], (u8, u8)> = HashMap::new();
    let mut compressed_data: Vec<u8> = Vec::new();

    while index < data.len() {
        while index > WINDOW_SIZE * move_window {
            move_window += 1;
        }

        let window_end = if WINDOW_SIZE * move_window > data.len() {
            data.len()
        } else {
            WINDOW_SIZE * move_window
        };

        let look_ahead_buf: &[u8] = &data[index..window_end];
        match find_prefix(&data, &dict, look_ahead_buf) {
            None => {
                dict.insert(&look_ahead_buf[0..1], (index as u8, 1));

                compressed_data.push(0);
                compressed_data.push(look_ahead_buf[0]);
                index += 1;
            }
            Some((pos, length)) => {
                let offset = index - pos as usize;
                dict.insert(&data[index..index + length as usize], (pos, length));

                compressed_data.push(offset as u8);
                compressed_data.push(length);

                index += length as usize;
            }
        }

    }

    compressed_data
}

fn find_prefix(data: &[u8], dict: &HashMap<&[u8], (u8, u8)>, look_ahead_buf: &[u8]) -> Option<(u8, u8)> {
    let mut length = 1;
    while length < look_ahead_buf.len() && dict.contains_key(&look_ahead_buf[..length]) {
        length += 1;
    }

    if length == 1 {
        None
    } else {
        let mut index = length - 1;
        while index < look_ahead_buf.len() && look_ahead_buf[index] == data[index] {
            index += 1;
        }
        let pos = dict.get(&look_ahead_buf[..length - 1]).unwrap().0;
        Some((pos, index as u8))
    }
}

fn lz77_decode(data: &[u8]) -> Vec<u8> {
    let mut decoded_message = vec![];
    let mut i = 0;
    while i < data.len() - 1 {
        if data[i] == 0 {
            decoded_message.push(data[i + 1]);
        } else {
            let offset = data[i] as usize;
            let length = data[i + 1] as usize;
            let start = decoded_message.len() - offset;
            for j in start..(start + length) {
                decoded_message.push(decoded_message[j]);
            }
        }
        i += 2;
    }
    decoded_message
}
