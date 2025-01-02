pub struct OWNSHA256;

impl OWNSHA256 {
    // Padding: ensure that the input data is of a certain length
    // padding scheme is PKCS#7 - bytes == u8
    #[warn(dead_code)]
    pub fn padding(data: &str, block_size: usize) -> Vec<u8> {
        if data.is_empty() {
            return vec![0; 64];
        }

        // Convert the string to ASCII values
        let mut ascii_values = data.as_bytes().to_vec();
        let original_len_bits = (ascii_values.len() * 8) as u64;

        // Append the '1' bit (0x80 in hex)
        ascii_values.push(0x80);

        // Append '0' bits until the length is congruent to 56 (mod 64)
        while (ascii_values.len() % block_size) != (block_size - 8) {
            ascii_values.push(0);
        }

        // Append the original length in bits as a 64-bit big-endian integer
        ascii_values.extend_from_slice(&original_len_bits.to_be_bytes());

        ascii_values
    }

    // Circular right shift
    // example of it = 0b1101 -> 0b0111, 0b1101 -> 0b1011, 0b1101 -> 0b1110
    // shifts the bits of a number to the right, with the bits that fall off being reintroduced on the left
    // Adds properties such as diffusion and resistance to collisions
    // small changes in the input produce significantly different outputs.
    // harder to reverse-engineer or predict the output.
    // rrb = right rotate bits
    fn rrb(x: u32, n: u32) -> u32 {
        (x >> n) | (x << (32 - n))
    }

    // Message schedule array generator
    // 512-bit blocks. Each 512-bit block is divided into 16 words, each 32 bits (4 bytes) long.
    // first 16 words are directly taken from the input block, and the remaining
    // 48 words are generated using specific functions
    #[warn(dead_code)]
    pub fn message_schedule(block_padded: &[u8]) -> [u32; 64] {
        assert_eq!(block_padded.len(), 64, "block_padded must be 64 bytes long");

        let mut words = [0u32; 64];

        // Initialize the first 16 words
        for i in 0..16 {
            words[i] = u32::from_be_bytes([
                block_padded[i * 4],
                block_padded[i * 4 + 1],
                block_padded[i * 4 + 2],
                block_padded[i * 4 + 3],
            ]);
        }

        // Generate the remaining 48 words
        for i in 16..64 {
            // positions i - 15 and i - 2 are used to access these previous words and apply bitwise operations to generate new words
            let sigma0 = OWNSHA256::rrb(words[i - 15], 7)
                ^ OWNSHA256::rrb(words[i - 15], 18)
                ^ (words[i - 15] >> 3);
            let sigma1 = OWNSHA256::rrb(words[i - 2], 17)
                ^ OWNSHA256::rrb(words[i - 2], 19)
                ^ (words[i - 2] >> 10);

            words[i] = words[i - 16]
                .wrapping_add(sigma0)
                // ensures that bits from earlier words are mixed into the current word.
                // This helps in spreading the influence of each input bit to many output bits, achieving diffusion.
                .wrapping_add(words[i - 7])
                .wrapping_add(sigma1);
        }

        words
    }
}

#[cfg(test)]
mod tests {
    use crate::encrypting::sha256::OWNSHA256;

    #[test]
    fn test_padding() {
        let data = "hello";
        let block_size = 64;

        let padded_data = OWNSHA256::padding(data, block_size);
        assert_eq!(padded_data.len(), 64); // Padded data should be 64 bytes
        assert_eq!(padded_data[0], 104); // 'h' in ASCII
        assert_eq!(padded_data[1], 101); // 'e' in ASCII
        assert_eq!(padded_data[5], 0x80); // '1' bit in padding
        assert_eq!(padded_data[6..56], [0; 50]); // '0' bits in padding
        assert_eq!(padded_data[56..], [0, 0, 0, 0, 0, 0, 0, 40]); // Length in bits

        let padded_data = OWNSHA256::padding("", block_size);
        assert_eq!(padded_data.len(), 64); // Padded data should be 64 bytes
        assert_eq!(padded_data[1..56], [0; 55]); // '0' bits in padding
        assert_eq!(padded_data[56..], [0, 0, 0, 0, 0, 0, 0, 0]); // Length in bits

        let char = "h";
        let padded_data = OWNSHA256::padding(char, block_size);
        assert_eq!(padded_data.len(), 64); // Padded data should be 64 bytes
        assert_eq!(padded_data[0], 104); // 'h' in ASCII
        assert_eq!(padded_data[1], 0x80); // '1' bit in padding
        assert_eq!(padded_data[2..56], [0; 54]); // '0' bits in padding
        assert_eq!(padded_data[56..], [0, 0, 0, 0, 0, 0, 0, 8]); // Length in bits
    }

    #[test]
    fn test_message_schedule() {
        let data = "hello";
        let block_size = 64;
        let padded_data = OWNSHA256::padding(data, block_size);
        let w = OWNSHA256::message_schedule(&padded_data);

        assert_eq!(w[0], 0x68656c6c); // 'hell' in ASCII
        assert_eq!(w[1], 0x6f800000); // 'o' followed by padding
        assert_eq!(w[2..14], [0; 12]); // Remaining words are zero
        assert_eq!(w[14], 0); // '0' bits in padding
        assert_eq!(w[15], 40); // Length in bits
    }
}
