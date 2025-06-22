fn main() {
    println!("HashMaps");
    test3::hash_tables::main_hash();

    println!("HashMap theory: Insertion time is O(1) on average, but can degrade to O(n) in the worst case if many elements hash to the same bucket. This is due to collisions, which can lead to longer search times as elements are stored in a linked list or similar structure within the bucket.");

    println!("-----------------------------------------------------------------------");

    println!("Adjacency List");
    test3::graphs::adjacency_list_main();
    println!("Adjacency List theory: An adjacency list is a collection of lists or arrays that represent a graph. Each list corresponds to a vertex and contains the vertices that are adjacent to it. This representation is efficient for sparse graphs, where the number of edges is much less than the square of the number of vertices, as it uses less space compared to an adjacency matrix.");

    println!("-----------------------------------------------------------------------");

    println!("Adjacency Matrix");
    test3::graphs::adjaceny_matrix_main();
    println!("Adjacency Matrix theory: An adjacency matrix is a 2D array where the element at row i and column j indicates whether there is an edge between vertex i and vertex j. This representation is efficient for dense graphs, where the number of edges is close to the square of the number of vertices, as it allows for quick edge existence checks but uses more space compared to an adjacency list.");

    println!("-----------------------------------------------------------------------");

    println!("Graphs");
    test3::graphs::graph_main();

    println!("Graph theory: A graph is a collection of nodes (or vertices) and edges that connect pairs of nodes. Graphs can be directed or undirected, weighted or unweighted, and can represent various structures such as networks, trees, and more. They are used in many applications, including social networks, transportation systems, and computer networks.");
    println!("BFS uses a queue to explore nodes level by level, ensuring that all nodes at the current depth are visited before moving deeper. It is useful for finding the shortest path in unweighted graphs. DFS uses a stack (or recursion) to explore as far as possible along each branch before backtracking, which can be more memory efficient for deep graphs but may not find the shortest path in unweighted graphs.");
    println!("DFS uses LIFO (Last In, First Out) structure, typically implemented with a stack or recursion, to explore as deep as possible into the graph before backtracking. BFS uses FIFO (First In, First Out) structure, typically implemented with a queue, to explore all neighbors at the present depth prior to moving on to nodes at the next depth level.");

    println!("-----------------------------------------------------------------------");

    println!("MST");
    test3::mst::mst_main();

    println!("MST theory: A Minimum Spanning Tree (MST) is a subset of the edges of a connected, undirected graph that connects all the vertices together without any cycles and with the minimum possible total edge weight. MST algorithms like Prim's and Kruskal's are used to find such trees efficiently. They are useful in network design, clustering, and other applications where minimizing cost while maintaining connectivity is important.");
    println!("Prim's algorithm starts with a single vertex and grows the MST by adding the smallest edge that connects a vertex in the MST to a vertex outside it, ensuring that no cycles are formed. Kruskal's algorithm sorts all edges and adds them one by one to the MST, ensuring that no cycles are formed, until all vertices are connected. Both algorithms have different use cases and performance characteristics depending on the graph structure.");
    println!("Kruskal's algorithm uses a disjoint-set data structure to efficiently manage and merge sets of vertices, ensuring that no cycles are formed while adding edges. It sorts the edges by weight and adds them in increasing order, checking for cycles using the disjoint-set structure. Prim's algorithm, on the other hand, grows the MST from a starting vertex by always adding the smallest edge that connects a vertex in the MST to a vertex outside it, using a priority queue to efficiently find the next edge to add.");

    println!("-----------------------------------------------------------------------");

    println!("Dijksta's");
    test3::dijkstra::dijkstra_main();
    println!("Dijkstra's algorith theory: Dijkstra's algorithm is a graph search algorithm that finds the shortest path from a source vertex to all other vertices in a weighted graph with non-negative edge weights. It uses a priority queue to explore the closest unvisited vertex, updating the shortest known distances to its neighbors. The algorithm guarantees that once a vertex is visited, the shortest path to it has been found. It is widely used in routing and navigation systems.");

    println!("-----------------------------------------------------------------------");

    println!("Bellman-Ford Algorithm");
    test3::bellman_ford::bellman_ford_main();
    println!("Bellman-Ford algorithm theory: The Bellman-Ford algorithm is a graph search algorithm that computes the shortest paths from a single source vertex to all other vertices in a weighted graph, allowing for negative edge weights. It works by repeatedly relaxing the edges, updating the shortest known distances, and can detect negative weight cycles. It is useful in scenarios where graphs may have negative weights, such as in financial networks or certain optimization problems.");

    println!("-----------------------------------------------------------------------");

    println!("Floyd-Warshall Algorithm");
    test3::floyd_warshall::floyd_warshall_main();
    println!("Floyd-Warshall algorithm theory: The Floyd-Warshall algorithm is a dynamic programming algorithm used to find the shortest paths between all pairs of vertices in a weighted graph. It works by iteratively improving the shortest path estimates by considering each vertex as an intermediate point. The algorithm can handle negative edge weights but cannot handle negative cycles. It is particularly useful for dense graphs and provides a complete distance matrix as output.");

    println!("-----------------------------------------------------------------------");

    println!("Ford-Fulkerson Algorithm");
    test3::ford_fulkerson::ford_fulkerson_main();
    println!("Ford-Fulkerson algorithm theory: The Ford-Fulkerson algorithm is a method for computing the maximum flow in a flow network. It uses augmenting paths to increase the flow until no more augmenting paths can be found. The algorithm can handle networks with integer capacities and is often implemented using the Edmonds-Karp algorithm, which uses BFS to find augmenting paths. It is widely used in network flow problems, such as transportation and logistics optimization.");
}

