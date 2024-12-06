use std::collections::{HashSet, VecDeque};
use petgraph::graph::UnGraph;


pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
    let n = graph.len(); 
    let all_visited = (1 << n) - 1; 
    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new(); 
    let mut visited: HashSet<i32> = HashSet::new(); 

    for i in 0..n as i32 { 
        queue.push_back((1 << i, i, 0)); 
        visited.insert((1 << i) * 16 + i); 
    } 

    while let Some((mask, node, dist)) = queue.pop_front() { 
        if mask == all_visited as i32 { 
            return dist; 
        } 

        for &neighbor in &graph[node as usize] { 
            let new_mask = mask | (1 << neighbor); 
            let hash_value = new_mask * 16 + neighbor; 

            if visited.insert(hash_value) { 
                queue.push_back((new_mask, neighbor, dist + 1)); 
            } 
        } 
    } 

    -1 

}

#[test]
fn test_addresses_no_weight_or_neighbors(){
    let _addresses = vec![
        7135, 7125, 7115,
        7105, 7059, 7053,
        7047, 7041, 7035,
        7030, 7036, 7040,
        7040, 7048, 7054, 
        7120, 7126, 7132,
        7138, 7144
        ];    
}

#[test]
fn test_addresses_unweighted(){
    let nw_graph = UnGraph::<i32, ()>::from_edges(&[
        (7135, 7125), (7125, 7115),
        (7115, 7105), (7105, 7059), 
        (7059, 7053), (7053, 7047),
        (7047, 7041), (7041, 7035),
        (7030, 7036), (7036, 7040),
        (7040, 7048), (7048, 7054),
        (7054, 7120), (7120, 7126),
        (7126, 7132), (7132, 7138),
        (7138, 7144),
    ]);    
}

#[test]
fn test_addresses_weighted() {

    let w_graph = UnGraph::<i32, ()>::from_edges(&[
        (7135, 7125, 1), (7125, 7115, 1),
        (7115, 7105, 1), (7105, 7059, 1), 
        (7059, 7053, 1), (7053, 7047, 1),
        (7047, 7041, 1), (7041, 7035, 1),
        (7030, 7036, 1), (7036, 7040, 1),
        (7040, 7048, 1), (7048, 7054, 1),
        (7054, 7120, 1), (7120, 7126, 1),
        (7126, 7132, 1), (7132, 7138, 1),
        (7138, 7144, 1),
    ]);    
}

#[test]
fn test_addresses_zigzag() {
    let zigzag = UnGraph::<i32, ()>::from_edges(&[
        (6940, 7010), (7010, 7005),
        (7005, 7025),
        (7025, 7030), (7030, 7036),
        (7036, 7040), (7040, 7048),
        (7048, 7054), (7054, 7038),
        (7038, 7041), (7041, 7047), (7047, 7053),
        (7053, 7059),
    ]);
}

fn main() {

}