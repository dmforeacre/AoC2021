//use std::rc::Rc;
//use std::cell::RefCell;
//use std::time::Instant;
//use std::collections::HashMap;
use crate::OutputStruct;
/*
#[derive(Clone, Debug)]
struct TreeNode {
    pub name: String,
    pub size: i32,
    pub children: RefCell<HashMap<String, Rc<TreeNode>>>,
    pub parent: Option<RefCell<Rc<TreeNode>>>,
}

impl TreeNode {
    pub fn new() -> TreeNode {
        return TreeNode { name: "".to_string(), size: (0), children: RefCell::new(HashMap::new()), parent: None }
    }
}*/

pub fn pt1() -> OutputStruct::OutputStruct {
    let mut output = OutputStruct::new();
    /*let mut clock = Instant::now();

    let string = std::fs::read_to_string("src/day7/input.txt").expect("Open failed");
    let mut lines = string.trim().split_terminator('\n');


    output.parseTime = clock.elapsed().as_micros();
    clock = Instant::now();

    let mut line = lines.next();
    let mut tokens: Vec<&str> = line.unwrap().split(" ").collect();
    let mut inListing = false;

    let root = Rc::new(TreeNode::new());
    let mut currNode = Rc::clone(&root);
    while line.is_some() {
        println!("{:?}", line);
        if tokens[0] == "$" {
            if tokens[1] == "cd" {
                if tokens[2] == ".." {
                    //println!("Moving to {:?}", currNode.name);
                    println!("{:?}", currNode);
                    currNode = Rc::clone(&currNode.parent.clone().unwrap().into_inner());
                }
                else if tokens[2] == "/" {
                    currNode = Rc::clone(&root);
                }
                else {
                    println!("Looking for {:?}", tokens[2]);
                    let newDir = currNode.children.borrow().get(tokens[2]).unwrap().clone();
                    currNode = newDir;
                }
            }
            else if tokens[1] == "ls" {
                inListing = true;
                line = lines.next();
                tokens = line.unwrap().split(" ").collect();
                while line.is_some() && tokens[0] != "$" {
                    println!("Adding {:?} to {:?}", tokens[1], currNode);
                    let mut newNode = TreeNode::new();
                    newNode.name = tokens[1].to_string();
                    if tokens[0] != "dir" {
                        newNode.size = tokens[0].parse::<i32>().unwrap();
                    }
                    let newParent = Some(RefCell::new(Rc::clone(&currNode)));
                    newNode.parent.replace(newParent.unwrap());
                    currNode.children.borrow_mut().insert(tokens[1].to_string(), Rc::new(newNode.clone()));
                    line = lines.next();
                    if line.is_some() {
                        tokens = line.unwrap().split(" ").collect();                    
                    }
                    println!("{:#?}", newNode.clone());
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
    //output.answer = total.to_string();*/
    output.answer = "Can't do in Rust".to_string();

    return output;
}