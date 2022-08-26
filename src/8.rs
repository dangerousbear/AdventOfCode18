mod parse_utils;

fn main() {
    let numbers = parse_utils::parse_int_list("data/8.txt", " ");
    println!("Numbers : {:?}", numbers);
    let (total_sum, _) = parse_node(&numbers);
    println!("Result part 1: {:?}", total_sum);
    let (total_value, _) = parse_node2(&numbers);
    println!("Result part 2: {:?}", total_value);
}

fn parse_node(slice: &[i32]) -> (i32, usize) {
    let mut total_metadata_sum = 0;
    let mut total_node_size = 2;
    let num_sub_nodes = slice[0];
    let num_metadata_entries = slice[1];
    for _ in 0..num_sub_nodes {
        let (node_metadata_sum, node_size) = parse_node(&slice[total_node_size..]);
        total_metadata_sum += node_metadata_sum;
        total_node_size += node_size;
    }
    for _ in 0..num_metadata_entries {
        total_metadata_sum += slice[total_node_size];
        total_node_size += 1;
    }

    (total_metadata_sum, total_node_size)
}

fn parse_node2(slice: &[i32]) -> (i32, usize) {
    let mut total_value = 0;
    let mut total_node_size = 2;
    let num_sub_nodes = slice[0];
    let num_metadata_entries = slice[1];
    let mut subnode_values = Vec::new();
    for _ in 0..num_sub_nodes {
        let (node_value, node_size) = parse_node2(&slice[total_node_size..]);
        subnode_values.push(node_value);
        total_node_size += node_size;
    }
    for _ in 0..num_metadata_entries {
        let entry = slice[total_node_size];
        if num_sub_nodes == 0 {
            total_value += entry;
        } else if entry >= 1 && (entry as usize) <= subnode_values.len() {
            total_value += subnode_values[(entry - 1) as usize];
        }
        total_node_size += 1;
    }
    (total_value, total_node_size)
}
