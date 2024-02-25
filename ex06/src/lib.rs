#![no_std]

// Use system crates
use core::ptr;

///! This tarack custom interface allocation.
///

struct Node {
    next: Option<*mut Node>,
    prev: Option<*mut Node>,
    data: u8,
}

struct NodeList {
    start: Option<*mut Node>,
    end: Option<*mut Node>,
}

const NODE_DEFAULT: Node = Node {
    next: None,
    prev: None,
    data: 0,
};

const NODE_LIST_DEFAULT: NodeList = NodeList {
    start: None,
    end: None,
};

static mut NODE_ARRAY: [Node; 5] = [NODE_DEFAULT; 5];
static mut FREE_LIST: NodeList = NODE_LIST_DEFAULT;
static mut ALLOCATED_LIST: NodeList = NODE_LIST_DEFAULT;

pub fn init_node_list() {
    unsafe {
        let mut prev_node = &mut NODE_ARRAY[0];
        NODE_ARRAY[0].prev = None;
        FREE_LIST.start = Some(prev_node);

        for i in 1..5 {
            prev_node.next = Some(&mut NODE_ARRAY[i]);
            NODE_ARRAY[i].prev = Some(prev_node);

            prev_node = &mut NODE_ARRAY[i];
        }

        prev_node.next = None;
        FREE_LIST.end = Some(prev_node);

        ALLOCATED_LIST.start = None;
        ALLOCATED_LIST.end = None;
    }
}

fn alloc_node() -> Option<*mut Node> {
    unsafe {
        let mut node = FREE_LIST.start;

        if node == None {
            return None;
        }

        // Update free list
        if let Some(n_ptr) = node {
            FREE_LIST.start = ptr::read(n_ptr).next;
        }

        if FREE_LIST.end == node {
            FREE_LIST.end = None;
            FREE_LIST.start = None;
        }

        // set default pointer inside node.
        (*node.unwrap()).next = None;
        (*node.unwrap()).prev = None;

        // Update allocated list
        if ALLOCATED_LIST.start == None {
            ALLOCATED_LIST.start = node;
            ALLOCATED_LIST.end = node;
        } else {
            if let Some(prev_node) = ALLOCATED_LIST.end {
                ptr::read(prev_node).next = node;
                ptr::read(node.unwrap()).prev = Some(prev_node);
            }
        }

        node
    }
}

fn free_node(node: Option<*mut Node>) {
    unsafe {
        if let Some(n_addr) = node {
            if (n_addr >= &mut NODE_ARRAY[0]) && (n_addr <= &mut NODE_ARRAY[4]) {
                let mut next_node: Option<*mut Node> = None;
                let mut prev_node: Option<*mut Node> = None;

                if ALLOCATED_LIST.start == ALLOCATED_LIST.end {
                    ALLOCATED_LIST.start = None;
                    ALLOCATED_LIST.end = None;
                } else if ALLOCATED_LIST.start == node {
                    next_node = (*node.unwrap()).next;
                    (*next_node.unwrap()).prev = None;
                    ALLOCATED_LIST.start = next_node;
                } else if ALLOCATED_LIST.end == node {
                    prev_node = (*node.unwrap()).prev;
                    (*prev_node.unwrap()).next = None;
                    ALLOCATED_LIST.end = prev_node;
                } else {
                    next_node = (*node.unwrap()).next;
                    prev_node = (*node.unwrap()).prev;

                    (*next_node.unwrap()).prev = prev_node;
                    (*prev_node.unwrap()).next = next_node;
                }

                if FREE_LIST.start == None {
                    FREE_LIST.start = node;
                    FREE_LIST.end = node;
                } else {
                    let mut end_node = FREE_LIST.end;

                    (*end_node.unwrap()).next = node;
                    (*node.unwrap()).next = None;
                    (*node.unwrap()).prev = end_node;
                    FREE_LIST.end = node;
                }
            }
        }
    }
}

pub struct instanceArray {
    alloc_block: Option<*mut Node>,
}

impl instanceArray {
    pub fn new() -> Self {
        instanceArray {
            alloc_block: alloc_node(),
        }
    }

    pub fn check_allocation(&self) -> bool {
        unsafe {
            let mut node = ALLOCATED_LIST.start;

            while let Some(n_ptr) = node {
                if n_ptr == self.alloc_block.unwrap() {
                    return true;
                }

                node = (*node.unwrap()).next;
            }
        }

        false
    }
}

impl Drop for instanceArray {
    fn drop(&mut self) {
        free_node(self.alloc_block);
    }
}

pub fn check_alloc_empty() -> bool {
    unsafe {
        if ALLOCATED_LIST.start == None {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloc_free_max() {
        let mut node_lst: [Option<*mut Node>; 5] = [None; 5];
        init_node_list();

        for idx in 0..5 {
            let node = alloc_node();
            node_lst[idx] = node;
            assert_ne!(node, None, "Fail at {} index", idx);
        }

        let node_last = alloc_node();
        assert_eq!(node_last, None, "Last alloc call not return None");

        for node in node_lst {
            free_node(node);
        }

        for idx in 0..5 {
            let node = alloc_node();
            assert_ne!(node, None, "Fail free - alloc logic {}", idx)
        }

        for idx in 0..5 {
            let node = node_lst[4 - idx];
            free_node(node);
        }

        for idx in 0..5 {
            let node = alloc_node();
            assert_ne!(node, None, "Fail free - alloc logic {}", idx)
        }

        for idx in 0..3 {
            let node = node_lst[3 - idx];
            free_node(node);
        }
        free_node(node_lst[0]);
        free_node(node_lst[4]);

        for idx in 0..5 {
            let node = alloc_node();
            assert_ne!(node, None, "Fail free - alloc logic {}", idx)
        }

        for node in node_lst {
            free_node(node);
        }
    }
}
