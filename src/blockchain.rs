use crate::block::Block;
use crate::transaction::Transaction;
use serde_json;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,  // Lista de transações pendentes
    pub mining_reward: u64,  // Recompensa por minerar um bloco
}

impl Blockchain {
    pub fn new(difficulty: usize, mining_reward: u64) -> Blockchain {
        let genesis_block = Block::new(0, String::from("0"), vec![]);
        Blockchain {
            chain: vec![genesis_block],
            difficulty,
            pending_transactions: vec![],
            mining_reward,
        }
    }

    // Função para adicionar uma transação pendente
    pub fn add_transaction(&mut self, sender: String, recipient: String, amount: u64) {
        let transaction = Transaction::new(sender, recipient, amount);
        self.pending_transactions.push(transaction);
    }

    // Função para adicionar um novo bloco à blockchain
    pub fn mine_pending_transactions(&mut self, mining_reward_address: String) {
        let reward_transaction = Transaction::new(String::from("Network"), mining_reward_address, self.mining_reward);
        self.pending_transactions.push(reward_transaction);  // Transação de recompensa

        let new_block = Block::new(self.chain.len() as u32, self.chain.last().unwrap().hash.clone(), self.pending_transactions.clone());
        let mut mined_block = new_block.clone();
        mined_block.mine(self.difficulty);

        self.chain.push(mined_block);
        self.pending_transactions.clear();  // Limpa as transações pendentes
    }

    // Função para imprimir a blockchain
    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("{:?}", block);
        }
    }

    // Função para validar o hash de um bloco
    fn is_valid_block(block: &Block, previous_block: &Block) -> bool {
        if block.hash != block.calculate_hash() {
            return false;
        }
        if block.previous_hash != previous_block.hash {
            return false;
        }
        true
    }

    // Função para validar toda a cadeia de blocos
    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if !Blockchain::is_valid_block(current_block, previous_block) {
                return false;
            }
        }
        true
    }

    // Função para salvar a blockchain em um arquivo
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&self).unwrap();
        std::fs::write(filename, serialized)
    }

    // Função para carregar a blockchain de um arquivo
    pub fn load_from_file(filename: &str) -> std::io::Result<Blockchain> {
        let data = std::fs::read_to_string(filename)?;
        let blockchain: Blockchain = serde_json::from_str(&data).unwrap();
        Ok(blockchain)
    }
}
