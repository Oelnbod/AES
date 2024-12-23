//DON'T FORGET TO GIT COMMIT
//AES/Rijndael implementation in rust

fn main() {
    let plaintext: String = "dbgghskhreakhreiohraeheaobsdajkgasdbkhello".to_string();
    let plaintext = padding(plaintext); //needed as aes is block cipher
    let segmented = split_into_segments(plaintext); //splitting into blocks
    let ascii_segment = convert_to_ascii(segmented[0].clone()); //needed for adding blocks
    println!("{:?}", ascii_segment);
}

fn padding(mut string: String) -> String {
    let mut is_padded = false;
    while is_padded == false {
        let length = string.len();
        if length % 16 != 0 {
            string.push_str("X"); //chosen this char for easy debugging and clear. could change later
        } else {
            is_padded = true;
        }
    }

    string
}

fn split_into_segments(string: String) -> Vec<String> {
    //this just iterates. print end_vale and start_value to understand what's happening
    let length = string.len();
    let num_segments = length / 16;
    let mut list_segments: Vec<String> = Vec::new();
    let mut start_value = 0;
    for segment in 0..num_segments {
        let end_value = (segment + 1) * 16;
        let segment: &str = &string[start_value..end_value];
        list_segments.push(segment.to_string());
        start_value = start_value + 16;
    }
    list_segments
}

fn convert_to_ascii(section: String) -> Vec<u8> {
    let block = section.chars(); //this returns an iterator (not a vec)
    let mut binary_block: Vec<u8> = Vec::new();
    for item in block {
        let decimal = item as u8;

        binary_block.push(decimal);
    }

    binary_block
}
