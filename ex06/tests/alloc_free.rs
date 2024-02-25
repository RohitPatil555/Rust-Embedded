use ex06::{check_alloc_empty, init_node_list, instanceArray};

#[test]
fn alloc_test() {
    init_node_list();

    let ia = instanceArray::new();

    assert_eq!(
        ia.check_allocation(),
        true,
        "Allocation block not found in list"
    );
}

#[test]
fn alloc_test_2() {
    assert_eq!(
        check_alloc_empty(),
        true,
        "Something wrong allocation list not empty"
    );
}
