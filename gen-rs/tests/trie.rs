use gen_rs::{Trie,Addr};


#[test]
fn test_trie() {
    let mut trie = Trie::<i64>::new();
    trie.insert_value("1 => 2", 1);
    trie.insert_value("1", 2);
    trie.insert_value("1 => 3", 3);
    trie.insert_value("1 => 3 => 4", 4);
    assert_eq!(trie.get_internal_node("1").unwrap().get_leaf_node("2"), trie.get_leaf_node("1 => 2"));
    assert_eq!(
        trie.get_internal_node("1").unwrap()
            .get_internal_node("3").unwrap()
            .get_leaf_node("4"),
        trie.get_leaf_node("1 => 3 => 4")
    );

    let mut subtrie = Trie::<i64>::new();
    subtrie.insert_value("2", 1);
    subtrie.insert_value("3", 3);
    subtrie.insert_value("3 => 4", 4);

    let mut root = Trie::<i64>::new();
    root.insert_submap("1", subtrie);
    root.insert_value("1", 2);

    assert_eq!(root, trie);
}