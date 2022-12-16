use std::time::Instant;
use std::collections::HashMap;
use crate::OutputStruct;

#[derive(Clone, Debug)]
struct TreeNode {
    pub name: Option<String>,
    pub size: i32,
    pub children: HashMap<String, Box<TreeNode>>,
    pub parent: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new() -> TreeNode {
        return TreeNode { name: None, size: (0), children: HashMap::new(), parent: None }
    }
}

pub fn pt1() -> OutputStruct::OutputStruct {
    let mut output = OutputStruct::new();
    output.answer = "Unfinished".to_string();
    return output;
    let mut clock = Instant::now();

    let string = std::fs::read_to_string("src/day7/input.txt").expect("Open failed");
    let mut lines = string.trim().split_terminator('\n');


    output.parseTime = clock.elapsed().as_micros();
    clock = Instant::now();

    let mut line = lines.next();
    let mut tokens: Vec<&str> = line.unwrap().split(" ").collect();
    let mut inListing = false;

    let mut root = TreeNode::new();
    root.name = Some("/".to_string());
    let mut currNode = root.clone();
    while line.is_some() {
        println!("{:?}", line);
        let mut currClone = currNode.clone();
        currClone.children = currNode.children.clone();
        if tokens[0] == "$" {
            if tokens[1] == "cd" {
                if tokens[2] == ".." {
                    println!("Moving to {:?}", currClone.parent.clone().unwrap().name);
                    
                    for c in currClone.children.clone() {
                        println!("Children: {:#?}", c.0);
                    }
                    currNode = *currClone.parent.clone().unwrap();
                }
                else if tokens[2] == "/" {
                    currNode = root.clone();
                }
                else {
                    for c in currClone.children.clone() {
                        println!("Children: {:#?}", c.0);
                    }
                    println!("Looking for {:?}", tokens[2]);
                    let childrenClone = currClone.children.clone();
                    let child = childrenClone.get(tokens[2]);
                    currNode = child.unwrap().as_ref().clone();
                }
            }
            else if tokens[1] == "ls" {
                inListing = true;
                line = lines.next();
                tokens = line.unwrap().split(" ").collect();
                while line.is_some() && tokens[0] != "$" {
                    let mut newNode = TreeNode::new();
                    newNode.name = Some(tokens[1].to_string());
                    newNode.parent = Some(Box::new(currNode.clone()));
                    let temp = newNode.clone();
                    println!("Setting {:?} parent to {:?}", temp.name, temp.parent.unwrap().name);
                    if tokens[0] != "dir" {
                        newNode.size = tokens[0].parse::<i32>().unwrap();
                    }
                    currClone.children.insert(newNode.name.clone().unwrap(), Box::new(newNode));
                    line = lines.next();
                    if line.is_some() {
                        tokens = line.unwrap().split(" ").collect();                    
                    }
                    currNode = currClone.clone();
                }
            }
        }
        if !inListing {
            line = lines.next();
            tokens = line.unwrap().split(" ").collect();
        }
        else {            
            inListing = false;
        }
    }

    output.calcTime = clock.elapsed().as_micros();
    //output.answer = total.to_string();

    return output;
}