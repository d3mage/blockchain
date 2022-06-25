pub mod blockchain {
    use base16;
    use chrono;
    use chrono::Utc;
    use core::fmt;
    use sha2::{Digest, Sha256};

    pub struct Block {
        index: u8,
        timestamp: chrono::DateTime<Utc>,
        proof: u128,
        prev_hash: u128,
    }

    impl Block {
        pub fn new(index: u8, proof: u128, prev_hash: u128) -> Block {
            Block {
                index: index,
                timestamp: chrono::offset::Utc::now(),
                proof: proof,
                prev_hash: prev_hash,
            }
        }

        pub fn get_hash(block: Block) -> String {
            let hash_string = block.to_string();
            let bytes = hash_string.as_bytes();
            let hash = Sha256::digest(bytes);
            base16::encode_lower(&hash)
        }
    }

    impl fmt::Display for Block {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Block number: {}\nTimestamp: {:?}\nProof: {}\nPrevious hash: {}\n",
                self.index, self.timestamp, self.proof, self.prev_hash
            )
        }
    }

    pub struct Blockchain {
        chain: Vec<Block>,
    }

    impl Blockchain {
        pub fn new() -> Blockchain {
            let mut chain: Vec<Block> = Vec::new();
            let genesis_block: Block = Block::new(0, 1, 0);
            chain.push(genesis_block);
            Blockchain { chain: chain }
        }

        pub fn add_new_block(&mut self, index: u8, proof: u128, prev_hash: u128) {
            let new_block: Block = Block::new(index, proof, prev_hash);
            self.chain.push(new_block);
        }

        pub fn get_last_block(&self) -> String {
            let length = self.chain.len();
            let last_block = &self.chain[length - 1];
            last_block.to_string()
        }
    }
}
