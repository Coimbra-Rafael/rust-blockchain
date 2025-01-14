use serde::{Serialize, Deserialize};
use sha2::{ Digest, Sha512};
use time::OffsetDateTime;
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub previous_hash: String,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,  // Lista de transações
    pub hash: String,
    pub nonce: u64, // O nonce para a prova de trabalho
}

impl Block {
    // Função para calcular o hash do bloco
    pub fn calculate_hash(&self) -> String {
        let block_string = format!(
            "{}{}{}{:?}{}",  // Modificado para incluir transações
            self.index, self.previous_hash, self.timestamp, self.transactions, self.nonce
        );
        let mut hasher = Sha512::new();
        hasher.update(block_string);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // Função para criar um novo bloco
    pub fn new(index: u32, previous_hash: String, transactions: Vec<Transaction>) -> Block {
        let timestamp = OffsetDateTime::now_utc().unix_timestamp() as u64;
        Block {
            index,
            previous_hash,
            timestamp,
            transactions,
            hash: String::new(),
            nonce: 0,
        }
    }

    // Função para realizar a Prova de Trabalho (PoW)
    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);  // Exemplo: "0000"
        while &self.calculate_hash()[..difficulty] != target {
            self.nonce += 1;
        }
        self.hash = self.calculate_hash();
    }
}
