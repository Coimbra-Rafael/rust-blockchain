use crate::blockchain::Blockchain;

mod block;
mod blockchain;
mod transaction;

fn main() {
    let difficulty: usize = 4;
    let mining_reward: u64 = 50;
    let mut my_blockchain: Blockchain = blockchain::Blockchain::new(difficulty, mining_reward);

    // Adicionando transações
    my_blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 30);
    my_blockchain.add_transaction(String::from("Bob"), String::from("Charlie"), 20);

    // Minerando o bloco
    my_blockchain.mine_pending_transactions(String::from("Miner1"));

    // Salvando a blockchain em um arquivo
    my_blockchain.save_to_file("blockchain.json").unwrap();

    // Carregando a blockchain do arquivo
    let loaded_blockchain = blockchain::Blockchain::load_from_file("blockchain.json").unwrap();

    // Imprime a blockchain carregada
    loaded_blockchain.print_chain();

    // Verifica se a blockchain carregada é válida
    if loaded_blockchain.is_chain_valid() {
        println!("Blockchain is valid!");
    } else {
        println!("Blockchain is invalid!");
    }
}
