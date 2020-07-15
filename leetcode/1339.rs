// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn prefix_sum(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(r) => {
                let left = prefix_sum(r.unwrap().get_mut().left);
                let right = prefix_sum(r.unwrap().get_mut().right);
                r.unwrap().get_mut().val + left + right
            },
            None => 0
        }
    }
    
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        println!("{}", prefix_sum(root));
        return 0;
    }
}