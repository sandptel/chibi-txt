use std::collections::{BinaryHeap,BTreeMap};
use std::{cmp::Reverse};

// https://www.geeksforgeeks.org/huffman-coding-greedy-algo-3/

type Link = Option<Box<Node>>;

/***************
NODE begin
 */
#[derive(Debug,Clone)]
struct Node{
    data: char,
    freq: usize,
    l: Link,
    r: Link,
    code:String,
}
impl PartialEq for Node{
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}
impl PartialOrd for Node{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.freq.partial_cmp(&other.freq)
    }
}
impl Eq for Node{

}
impl Ord for Node{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq.cmp(&other.freq)
    }
}
impl Node{
    fn new(data: char, freq: usize)->Node{
        Node {data:data,freq:freq,l:None,r:None,code:"".to_string()}
    }
}

/***************
NODE END
 */


fn build_huff_tree(nodes : &mut Vec<Node>) -> Node{
    // alg:
    // 1) Order the nodes based on freq
    // 2) Merge the two nodes with smallest freqs
    // 3) repeat until one node left
    while(nodes.len() > 1){
        let new_freq = nodes[0].freq + nodes[1].freq;
        let left_link = Some(Box::new(Node{
            data:nodes[0].data,freq:nodes[0].freq,
            l:nodes[0].l.clone(),r:nodes[0].r.clone(),code:"".to_string()
        }));
        let right_link = Some(Box::new(Node{
            data:nodes[1].data,freq:nodes[1].freq,
            l:nodes[1].l.clone(),r:nodes[1].r.clone(),code:"".to_string()
        }));
        // $ is special character
        let new_node = Node{data:'$',freq:new_freq,
            l:left_link,r:right_link,code:"".to_string()
        };

        // remove first two elements of vector
        nodes.drain(0..2);

        // push new node in 
        nodes.push(new_node);
        // sort the tree
        nodes.sort();
    }

    nodes[0].clone()
}

fn mark_tree(root: &mut Option<Box<Node>>,marker:&mut String){
    match root{
        Some(inside) => {
            marker.push('0');
            mark_tree(&mut inside.l,marker);
            inside.code = marker.to_string();
            marker.push('1');
            mark_tree(&mut inside.r,marker);
            marker.pop();
        }
        None => {
            // reached leaf     
            marker.pop();
        }
    }
}

fn main() {
    // let msg = "ABBAABABDA".to_string();

    // let hm = get_hash_char_freq(msg);

    // let nodes = get_nodes(hm);
    let nodeA =  Node::new('A',35);
    let nodeB =  Node::new('B',10);
    let nodeC =  Node::new('C',20);
    let nodeD =  Node::new('D',20);
    let node_ =  Node::new('_',15);
    let arr_nodes = [
        nodeA,nodeB,nodeC,nodeD,node_
    ];
    let mut nodes = Vec::from(arr_nodes);
    // sort in desc order
    nodes.sort();

    let mut tree_head = build_huff_tree(&mut nodes);
    let mut tree_head_ref = Some(Box::new(tree_head));
    mark_tree(&mut tree_head_ref,&mut "".to_string());
    println!("{:?}",tree_head_ref);
}

fn get_hash_char_freq(msg:String) -> BTreeMap<char,usize> {
    /*
    Return a hashmap of characters and their respective frequencies.
    ie:
    {
        'a': 2,
        'b':35,
        ...
    }
    */
    let mut ret_hash: BTreeMap<char,usize> = BTreeMap::new();
    
    for (_i,c) in msg.chars().enumerate(){
        if ret_hash.contains_key(&c){
            // if contains, just add to the freq
            ret_hash.insert(c,1+ret_hash[&c]);
        }else{
            // first entry
            ret_hash.insert(c,1);
        }
    }

    ret_hash
}

fn get_nodes(btm: BTreeMap<char,usize>) -> Vec<Node>{
    let mut vec : Vec<Node> = Vec::new();

    for (c,f) in btm.iter(){
        let node : Node = Node::new(*c,*f);
        vec.push(node);
    }

    vec.sort();

    vec
}