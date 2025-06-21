pub mod hash_tables {
    use std::collections::HashMap;

    pub fn main_hash(){
        let keys = vec![4322, 1334, 1471, 9679, 1989, 6171, 6173, 4199];
        let mut hash_table:HashMap<i32, Vec<i32>> = (0..10)
            .map(|i| (i, Vec::new()))
            .collect();

        let mut hash_table_copy = hash_table.clone();

        println!{"Hashing without Linear Probing"}
        compute_hash(&keys, &mut hash_table);

        println!{"Hashing with Linear Probing"}
        double_hashing(&keys, &mut hash_table_copy, 10);
    }
    pub fn compute_hash(hash_values: &[i32], hash_t: &mut HashMap<i32, Vec<i32>>) {

        for &value in hash_values {
            let hash = value % 10;
            insert_into_hash_table(hash_t, hash, value);
        }

        for (key, values) in hash_t.iter() {
            println!("Key: {}, Values: {:?}", key, values);
        }
    }

    pub fn double_hashing(hash_values: &[i32], hash_t: &mut HashMap<i32, Vec<i32>>, m: i32) {
        for &value in hash_values {
            let h1 = value % m;
            let h2 = 1 + (value % (m - 1));
            let mut i = 0;
            loop {
                let hash = (h1 + i * h2) % m;
                if hash_t.get(&hash).map_or(true, |v| v.is_empty()) {
                    insert_into_hash_table(hash_t, hash, value);
                    break;
                }
                i += 1;
            }
        }

        for (key, values) in hash_t.iter() {
            println!("Key: {}, Values: {:?}", key, values);
        }
    }

    pub fn insert_into_hash_table(hash_t: &mut HashMap<i32, Vec<i32>>, key: i32, value: i32) {
        hash_t.entry(key).or_insert(Vec::new()).push(value);
    }

}

pub mod graphs {
    use std::collections::HashMap;
    pub struct Node<T> {
        value: T,
    }

    pub struct Graph<T> {
        nodes: Vec<Node<T>>,
        adj: Vec<Vec<usize>>,
    }
    impl<T: std::fmt::Display> Graph<T>{
        pub fn new() -> Self {
            Graph { nodes: Vec::new(), adj: Vec::new() }
        }

        pub fn add_node(&mut self, value: T) {
            self.nodes.push(Node { value });
            self.adj.push(Vec::new());
        }

        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.adj[u].push(v);
            self.adj[v].push(u);
        }

        pub fn display(&self) {
            for node in &self.nodes {
                println!("{}", node.value);
            }
        }

        pub fn dfs(&self, start: usize) {
            let mut visited = vec![false; self.nodes.len()];
            self.dfs_helper(start, &mut visited);
        }

        fn dfs_helper(&self, node: usize, visited: &mut Vec<bool>) {
            visited[node] = true;
            println!("{}", self.nodes[node].value);
            for &neighbor in &self.adj[node] {
                if !visited[neighbor] {
                    self.dfs_helper(neighbor, visited);
                }
            }
        }

        pub fn bfs(&self, start: usize) {
            let mut visited = vec![false; self.nodes.len()];
            let mut queue = std::collections::VecDeque::new();
            visited[start] = true;
            queue.push_back(start);

            while let Some(node) = queue.pop_front() {
                println!("{}", self.nodes[node].value);
                for &neighbor in &self.adj[node] {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }
    pub fn adjacency_list_main(){
        let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();

        // Adding edges to the adjacency list
        add_edge(&mut adjacency_list, 1, 2);
        add_edge(&mut adjacency_list, 1, 3);
        add_edge(&mut adjacency_list, 2, 4);
        add_edge(&mut adjacency_list, 3, 5);

        // Displaying the adjacency list
        display_adjacency_list(&adjacency_list);
    }
    pub fn add_edge(adjacency_list: &mut HashMap<i32, Vec<i32>>, u: i32, v: i32) {
        adjacency_list.entry(u).or_insert(Vec::new()).push(v);
        adjacency_list.entry(v).or_insert(Vec::new()).push(u);
    }

    pub fn display_adjacency_list(adjacency_list: &HashMap<i32, Vec<i32>>) {
        for (key, values) in adjacency_list.iter() {
            println!("{}: {:?}", key, values);
        }
    }

    pub fn adjaceny_matrix_main(){
        let mut adjacency_matrix: Vec<Vec<i32>> = vec![vec![0; 5]; 5];

        // Adding edges to the adjacency matrix
        add_edge_matrix(&mut adjacency_matrix, 0, 1);
        add_edge_matrix(&mut adjacency_matrix, 0, 2);
        add_edge_matrix(&mut adjacency_matrix, 1, 3);
        add_edge_matrix(&mut adjacency_matrix, 2, 4);

        // Displaying the adjacency matrix
        display_adjacency_matrix(&adjacency_matrix);
    }

    pub fn add_edge_matrix(adjacency_matrix: &mut Vec<Vec<i32>>, u: i32, v: i32) {
        if u < adjacency_matrix.len() as i32 && v < adjacency_matrix.len() as i32 {
            adjacency_matrix[u as usize][v as usize] = 1;
            adjacency_matrix[v as usize][u as usize] = 1; // For undirected graph
        } else {
            println!("Error: Vertex out of bounds");
        }
    }

    pub fn display_adjacency_matrix(adjacency_matrix: &Vec<Vec<i32>>) {
        for row in adjacency_matrix {
            for value in row {
                print!("{} ", value);
            }
            println!();
        }
    }

    use std::fs::File;
    use std::io::Write;
    use std::process::Command;

    impl<T: std::fmt::Display> Graph<T> {
        pub fn to_dot(&self) -> String {
            let mut dot = String::from("graph G {\n");

            for (i, node) in self.nodes.iter().enumerate() {
                dot.push_str(&format!("  {} [label=\"{}\"];\n", i, node.value));
            }

            let mut added_edges = std::collections::HashSet::new();
            for (u, neighbors) in self.adj.iter().enumerate() {
                for &v in neighbors {
                    let edge = if u < v { (u, v) } else { (v, u) };
                    if !added_edges.contains(&edge) {
                        dot.push_str(&format!("  {} -- {};\n", u, v));
                        added_edges.insert(edge);
                    }
                }
            }

            dot.push_str("}\n");
            dot
        }

        pub fn visualize(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
            let dot_content = self.to_dot();
            let dot_file = format!("{}.dot", filename);
            let png_file = format!("{}.png", filename);

            let mut file = File::create(&dot_file)?;
            file.write_all(dot_content.as_bytes())?;

            Command::new("dot")
                .args(&["-Tpng", &dot_file, "-o", &png_file])
                .output()?;

            println!("Graph saved as {}", png_file);
            Ok(())
        }
    }

    pub fn graph_main(){
        let mut graph = Graph { nodes: Vec::new(), adj: vec![] };

        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(4);

        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 0);
        graph.add_edge(0, 2);

        println!("DFS Traversal:");
        graph.dfs(0);
        println!("BFS Traversal:");
        graph.bfs(0);

        println!("Displaying Graph Nodes:");
        graph.visualize("graph.dot").unwrap();
    }

}