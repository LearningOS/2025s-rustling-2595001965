/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/

use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    fn new(n: usize) -> Self {
        let mut adj = vec![vec![]; n];
        // 在初始化时对邻接表进行排序
        for neighbors in &mut adj {
            neighbors.sort();
        }
        Graph { adj }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        // 添加边时，确保src和dest在图的范围内
        if src < self.adj.len() && dest < self.adj.len() {
            self.adj[src].push(dest);
            self.adj[dest].push(src); 
            // 添加边后，对邻接表进行排序
            self.adj[src].sort();
            self.adj[dest].sort();
        } else {
            panic!("Invalid edge: src or dest out of bounds");
        }
    }

    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        if visited.contains(&v) {
            return;
        }
        
        visited.insert(v);
        visit_order.push(v);
        
        // 由于邻接表已经排序，直接遍历即可
        for &neighbor in &self.adj[v] {
            self.dfs_util(neighbor, visited, visit_order);
        }
    }

    // Perform a depth-first search on the graph, return the order of visited nodes
    fn dfs(&self, start: usize) -> Vec<usize> {
        // 确保起始节点在图的范围内
        if start >= self.adj.len() {
            panic!("Start node out of bounds");
        }

        let mut visited = HashSet::new();
        let mut visit_order = Vec::new(); 
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }

    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => return false, // 处理非法字符
            }
        }
        stack.is_empty() // 关键修复点：最后必须检查栈是否为空
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]); 
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]); 
    }
}