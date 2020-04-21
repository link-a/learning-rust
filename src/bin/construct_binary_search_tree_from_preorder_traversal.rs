// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

#[allow(unused)]
impl Solution {
    #[allow(unused)]
    pub fn from_preorder(preorder: &mut Vec<i32>, min:i32, max: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() < 1 {
            return None;
        }
        if preorder[0] < min || preorder[0] > max {
            return None;
        }
        let current = preorder.remove(0);
        let mut left = Solution::from_preorder(preorder, min, current);
        let mut right = Solution::from_preorder(preorder, current, max);

        let mut node = TreeNode {
            val: current,
            left: left,
            right: right,
        };
        let cell = RefCell::new(node);
        let root = Rc::new(cell);
        Some(root)
    }

    #[allow(unused)]
    pub fn bst_from_preorder(mut preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let answer =
            match preorder.len() {
                0 => None,
                _ => Solution::from_preorder(&mut preorder, std::i32::MIN, std::i32::MAX)
            };
        return answer;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    pub fn simple_test() {
        let mut input = vec![1];
        run_test(&mut input);
    }

    #[test]
    pub fn simple_test_bit_longer() {
        let mut input = vec![6,10,8,7,8,12];
        run_test(&mut input);
    }

    pub fn run_test(input: &mut Vec<i32>) {
        let test_input = input.to_vec();
        let actual = Solution::bst_from_preorder(test_input);
        let actual_pre_order = from_bst(actual);
        println!("{:?} vs {:?}", actual_pre_order, input.to_vec());
        assert_eq!(actual_pre_order, input.to_vec());
    }

    pub fn pre_order_traversal(root: &Option<Rc<RefCell<TreeNode>>>, answer: &mut Vec<i32>) {
        if let Some(root) = root {
            let root = root.borrow();
            answer.push(root.val);
            tests::pre_order_traversal(&root.left, answer);
            tests::pre_order_traversal(&root.right, answer);
        }
    }

    pub fn from_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer:Vec<i32> = Vec::new();
        tests::pre_order_traversal(&root, &mut answer);
        return answer;
    }
}

pub fn main() {
    println!("construct_binary_search_tree_from_preorder_traversal");
}