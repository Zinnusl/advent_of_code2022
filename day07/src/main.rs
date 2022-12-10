use std::fmt::{Display, Formatter};

use regex::Regex;

type FileSize = u32;

#[derive(Debug, PartialEq)]
struct FileTree {
    root: Node,
}

#[derive(Debug, PartialEq, Clone)]
struct NodeIndex(Vec<usize>);

impl NodeIndex {
    fn add(&mut self, index: usize) {
        self.0.push(index);
    }

    fn find_parent_of_index(&self) -> NodeIndex {
        if self.0.len() >= 1 {
            return NodeIndex(self.0[..self.0.len() - 1].to_vec());
        }

        NodeIndex(vec![])
    }
}

impl From<&Node> for NodeIndex {
    fn from(node: &Node) -> NodeIndex {
        match node {
            Node::File ( index, .. ) => index.clone(),
            Node::Dir ( index, .. ) => index.clone(),
        }
    }
}
impl From<&mut Node> for NodeIndex {
    fn from(node: &mut Node) -> NodeIndex {
        match node {
            Node::File ( index, .. ) => index.clone(),
            Node::Dir ( index, .. ) => index.clone(),
        }
    }
}

impl FileTree {
    fn root(&mut self) -> &mut Node {
        &mut self.root
    }

    fn get(&mut self, index: NodeIndex) -> Option<&mut Node> {
        let mut node = &mut self.root;

        for i in index.0 {
            match node {
                Node::File ( .. ) => return Some(node),
                Node::Dir ( .., nodes ) => {
                    node = nodes.get_mut(i)?;
                }
            }
        }

        Some(node)
    }
}

impl Display for FileTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::File ( index, name, size ) => {
                write!(f, "{}{} ({})\n", " ".repeat(index.0.len() * 2), name, size)
            }
            Node::Dir ( index, name, nodes ) => {
                writeln!(f, "{}{}", " ".repeat(index.0.len() * 2), name)?;

                for node in nodes {
                    write!(f, "{}", node)?;
                }

                Ok(())
            }
        }
    }
}

impl From<&str> for FileTree {
    fn from(s: &str) -> Self {
        let mut tree = FileTree { root: Node::Dir(NodeIndex(vec![]), "/".to_string(), vec![]) };

        let mut current = NodeIndex::from(tree.root());
        let re = Regex::new(r"\$? ?(?P<cmd>cd|dir|\d+) (?P<val>.+?)$").unwrap();
        for line in s.lines().skip(1) {
            println!("line: {}", line);

            for cap in re.captures_iter(line) {
                let first = &cap["cmd"];
                let second = &cap["val"];
                println!("first: {}", first);
                println!("second: {}", second);

                let current_node = match tree.get(current.clone()) {
                    Some(node) => node,
                    None => tree.root(),
                };

                match first {
                    "cd" => {
                        println!("cd {}", second);
                        current = current_node.cd(second);
                    }
                    "dir" => {
                        println!("dir {}", second);
                        current_node.create_dir(second);
                    }
                    x => {
                        println!("{} {}", x, second);
                        if let Ok(size) = x.parse::<FileSize>() {
                            current_node.create_file(second, size);
                        }
                        else {
                            panic!("Invalid input: {}", line);
                        }
                    }
                }
            }
        }


        tree
    }
}

#[derive(Debug, PartialEq)]
enum Node {
    File(NodeIndex, String, FileSize),
    Dir(NodeIndex, String, Vec<Node>),
}

impl Node {
    fn create_file(&mut self, name: &str, size: u32) -> &mut Node {
        match self {
            Node::Dir(index, _, nodes) => {
                let mut index = index.clone();
                index.add(nodes.len());
                nodes.push(Node::File(index, name.to_string(), size));
                nodes.last_mut().unwrap()
            }
            _ => panic!("not a directory"),
        }
    }
    fn cd<'a>(&'a mut self, name: &str) -> NodeIndex {
        match self {
            Node::Dir(index, .., nodes) => {
                if name == ".." {
                    return index.find_parent_of_index();
                }
                for (idx, node) in nodes.iter_mut().enumerate() {
                    match node {
                        Node::Dir(index, dirname, _) => {
                            if dirname == name {
                                let mut index = index.clone();
                                index.add(idx);
                                return index;
                            }
                        }
                        _ => {}
                    }
                }
                panic!("directory not found");
            }
            _ => panic!("not a directory"),
        }
    }
    fn create_dir(&mut self, dir: &str) -> &mut Node {
        match self {
            Node::Dir(index, .., nodes) => {
                let mut index = index.clone();
                index.add(nodes.len());
                nodes.push(Node::Dir(index, dir.to_string(), vec![]));
                nodes.last_mut().unwrap()
            }
            _ => panic!("not a directory"),
        }
    }
}

fn main() {
    // let input = std::fs::read_to_string("input.txt").expect("Could not read input file");

    // let tree = FileTree::from(input.as_str());

    // println!("{:?}", tree);

        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let tree = FileTree::from(input);

        println!("{}", tree);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let tree = FileTree::from(input);
        // assert_eq!(tree.root, Node::Dir(NodeIndex(vec![]), "/".to_string(), vec![
        //     Node::Dir(NodeIndex(vec![0]),"a".to_string(), vec![
        //         Node::Dir(NodeIndex(vec![0, 0]),"e".to_string(), vec![
        //             Node::File(NodeIndex(vec![0,0,0]),"i".to_string(), 584),
        //         ]),
        //         Node::File(NodeIndex(vec![0,1]),"f".to_string(), 29116),
        //         Node::File(NodeIndex(vec![0,2]),"g".to_string(), 2557),
        //         Node::File(NodeIndex(vec![0,3]),"h.lst".to_string(), 62596),
        //     ]),
        //     Node::File(NodeIndex(vec![1]),"b.txt".to_string(), 14848514),
        //     Node::File(NodeIndex(vec![2]),"c.dat".to_string(), 8504156),
        //     Node::Dir(NodeIndex(vec![3]),"d".to_string(), vec![
        //         Node::File(NodeIndex(vec![3,0]),"j".to_string(), 4060174),
        //         Node::File(NodeIndex(vec![3,1]),"d.log".to_string(), 8033020),
        //         Node::File(NodeIndex(vec![3,2]),"d.ext".to_string(), 5626152),
        //         Node::File(NodeIndex(vec![3,3]),"k".to_string(), 7214296),
        //     ]),
        // ]));
    }
}
