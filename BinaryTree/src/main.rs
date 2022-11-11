#![allow(non_snake_case)]

type ChildNode<T> = Option<Box<BTNode<T>>>;

struct BTNode<T>{
    left: ChildNode<T>,
    right: ChildNode<T>,
    op: Op<T>
}

enum Op<T>{
    Add,
    Sub,
    Div,
    Mul,
    Id(T)
}

struct BinaryTree<T>{
    head: Option<BTNode<T>>
}

impl BTNode<i32>{
    pub fn new(op: Op<i32>, l: BTNode<i32>, r: BTNode<i32>)->Self{
        BTNode::<i32>{
            op: op, left: Some(Box::new(l)), right: Some(Box::new(r))
        }
    }
}

fn AddNode(l: BTNode<i32>, r: BTNode<i32>)-> BTNode<i32>{
    BTNode::new(Op::Add, l, r)
}
fn SubNode(l: BTNode<i32>, r: BTNode<i32>)-> BTNode<i32>{
    BTNode::new(Op::Sub, l, r)
}
fn DivNode(l: BTNode<i32>, r: BTNode<i32>)-> BTNode<i32>{
    BTNode::new(Op::Div, l, r)
}
fn MulNode(l: BTNode<i32>, r: BTNode<i32>)-> BTNode<i32>{
    BTNode::new(Op::Mul, l, r)
}
fn IdNode(value: i32)-> BTNode<i32>{
    BTNode { left: None, right: None, op: Op::Id(value) }
}

impl BinaryTree<i32>{
    pub fn new(head: BTNode<i32>)->Self{
        BinaryTree::<i32>{head: Some(head)}
    }
    pub fn collapse(node: &Box<BTNode<i32>>)->i32{
        let mut r: Option<i32> = None;
        let mut l: Option<i32> = None;

        if let Some(left) = &node.left{
            l = Some(BinaryTree::collapse(left));
        }
        if let Some(right) = &node.right{
            r = Some(BinaryTree::collapse(right));
        }
        let r = if let Some(x) = r {x} else {0};
        let l = if let Some(x) = l {x} else {0};

        match node.op{
            Op::Add => {l+r}
            Op::Sub => {l-r}
            Op::Mul => {l*r}
            Op::Div => {
                if r==0{
                    panic!("attempted divide-by-zero operation.")
                }
                l/r
            }
            Op::Id(x) => x
        }
    }
}

fn main() {
    let bt = BinaryTree::new(
        AddNode(
            SubNode(
                IdNode(10),
                MulNode(
                    IdNode(2),
                    IdNode(2)
                )
            ),
            AddNode(
                IdNode(8),
                DivNode(
                    IdNode(10),
                    IdNode(2)
                )
            )
        )
    );

    println!("{}", BinaryTree::collapse(&Box::new(bt.head.expect("No head initialized."))))
}