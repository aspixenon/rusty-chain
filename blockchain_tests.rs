use rusty_chain::Blockchain;

#[test]
fn test_genesis_block() {
    let chain = Blockchain::new(1);
    assert_eq!(chain.chain.len(), 1);
    assert_eq!(chain.chain[0].index, 0);
}

#[test]
fn test_add_block() {
    let mut chain = Blockchain::new(1);
    chain.add_block(String::from("Test Data"));
    assert_eq!(chain.chain.len(), 2);
    assert_eq!(chain.chain[1].data, "Test Data");
}

#[test]
fn test_valid_chain() {
    let mut chain = Blockchain::new(1);
    chain.add_block(String::from("Block 1"));
    chain.add_block(String::from("Block 2"));
    assert!(chain.is_chain_valid());
}