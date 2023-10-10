use itertools::Itertools;
use sha3::{Digest, Sha3_256};
use hex_literal::hex;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn main() {
    let hash_task = [                                                                // hashes whose pre-images must be found
        hex!("a03ab19b866fc585b5cb1812a2f63ca861e7e7643ee5d43fd7106b623725fd67"),
        hex!("d182aed568b01fee105557a1d173791c798030db267cf94e17102b94dcbbda3c"),
        hex!("7b6a784b05c64d2e669e026fc61296eca2ee8acd5112eb8ae5f16023809e203b")
    ];
    let mut input_file = File::open("rockyou.txt").unwrap();                        // paste yourself wordlist
    let mut output_file = File::create("result.txt").unwrap();
    let mut find_count = 0u8;

    for line in io::BufReader::new(&mut input_file).lines() {
        if find_count == 3 {
            break;
        }

        let password = match line {
            Ok(password) => password,
            Err(_) => continue
        };

        let mut hasher = Sha3_256::new();
        hasher.update(&password);
        let result = hasher.finalize();
        if hash_task[0][..] == result[..] || hash_task[1][..] == result[..] || hash_task[2][..] == result[..] {
            find_count += 1;
            output_file.write_all(format!("{} {:02x?}", password, result.iter().format("")).as_bytes()).unwrap();
        }
        else {
            println!("{} {:02x?}", password, result.iter().format(""));
        }
    }
}
