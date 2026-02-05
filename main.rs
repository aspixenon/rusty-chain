use rusty_chain::Blockchain;

fn main() {
    let mut chain = Blockchain::new(2);
    
    chain.add_block(String::from("First Transaction"));
    chain.add_block(String::from("Second Transaction"));

    println!("{:?}", chain.chain);
    println!("Is chain valid? {}", chain.is_chain_valid());
}