#[test]
fn test() {
    const CACHE: [[u8; 32]; 2] = gen_merkletree_cache::generate!(2);
    println!("{:?}",CACHE);
}