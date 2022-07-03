pub mod blockchain {
    use base16;
    use chrono;
    use chrono::serde::ts_seconds;
    use chrono::Utc;
    use core::fmt;
    use serde::{Deserialize, Serialize};
    use sha2::{Digest, Sha256};

    #[derive(Serialize, Deserialize)]
    pub struct Block {
        index: u8,
        #[serde(with = "ts_seconds")]
        timestamp: chrono::DateTime<Utc>,
        nonce: u128,
        prev_hash: String,
    }

    impl Block {
        pub fn new(
            index: u8,
            timestamp: chrono::DateTime<Utc>,
            nonce: u128,
            prev_hash: String,
        ) -> Block {
            Block {
                index: index,
                timestamp: timestamp,
                nonce: nonce,
                prev_hash: prev_hash,
            }
        }

        pub fn hash_value(hash_string: String) -> String {
            let bytes = hash_string.as_bytes();
            let hash = Sha256::digest(bytes);
            base16::encode_lower(&hash)
        }

        pub fn get_hash(
            timestamp: chrono::DateTime<Utc>,
            nonce: u128,
            prev_hash: String,
        ) -> String {
            let mut hash_string = timestamp.to_string();
            hash_string.push_str(&nonce.to_string());
            hash_string.push_str(&prev_hash);
            Self::hash_value(hash_string)
        }

        pub fn mine_block(index: u8, last_block: &Block) -> Block {
            let last_hash = Self::hash_value(last_block.to_string());
            let mut is_valid = false;

            let mut timestamp = chrono::offset::Utc::now();
            let mut current_hash;
            let mut nonce = 0;

            while !is_valid {
                for i in 1..u128::MAX {
                    current_hash = Block::get_hash(timestamp, i, last_hash.clone());
                    let prefix = &current_hash[0..4];
                    if prefix.eq("0000") {
                        is_valid = true;
                        nonce = i;
                        break;
                    }
                }
                timestamp = chrono::offset::Utc::now();
            }
            Block {
                index: index,
                timestamp: timestamp,
                nonce: nonce,
                prev_hash: last_hash,
            }
        }
    }

    impl fmt::Display for Block {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Block number: {}\nTimestamp: {:?}\nProof: {}\nPrevious hash: {}\n",
                self.index, self.timestamp, self.nonce, self.prev_hash
            )
        }
    }

    pub struct Blockchain {
        pub chain: Vec<Block>,
    }

    impl Blockchain {
        pub fn new() -> Blockchain {
            let mut chain: Vec<Block> = Vec::new();
            let timestamp = chrono::offset::Utc::now();
            let genesis_block: Block = Block::new(0, timestamp, 1, "0x00".into());
            chain.push(genesis_block);
            Blockchain { chain: chain }
        }

        pub fn get_last_block(&self) -> &Block {
            let length = self.chain.len();
            &self.chain[length - 1]
        }

        pub fn mine_block(&mut self) -> &Block {
            let index = self.chain.len() as u8;
            let last_block = self.get_last_block();

            let new_block = Block::mine_block(index, &last_block);
            self.chain.push(new_block);
            self.get_last_block()
        }

        pub fn is_chain_valid(&self) -> bool {
            let mut current_block: &Block;
            let mut prev_block: &Block;
            for i in 1..self.chain.len() {
                current_block = &self.chain[i];
                prev_block = &self.chain[i - 1];
                let prev_hash = &Block::hash_value(prev_block.to_string());
                let curr_hash = &current_block.prev_hash;
                if prev_hash != curr_hash {
                    return false;
                }
            }
            true
        }
    }
    impl fmt::Display for Blockchain {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut current_block: &Block;
            let length = self.chain.len();
            for i in 0..length {
                current_block = &self.chain[i];
                write!(
                    f,
                    "Block number: {}\nTimestamp: {:?}\nNonce: {}\nPrevious hash: {}\n",
                    current_block.index,
                    current_block.timestamp,
                    current_block.nonce,
                    current_block.prev_hash
                )?;
            }
            write!(f, "Length of blockchain: {}", length)?;
            Ok(())
        }
    }
}
