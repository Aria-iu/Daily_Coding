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
impl Solution {
    pub fn f(root: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>,i32) {
        if root.is_none() {
            return (root,0);
        }

        let (l_r,l_ret) = Solution::f(root.as_ref().unwrap().borrow().left.clone());
        let (r_r,r_ret) = Solution::f(root.as_ref().unwrap().borrow().right.clone());

        if l_ret > r_ret {
            return (l_r, l_ret+1);
        }
        if r_ret > l_ret {
            return (r_r, r_ret+1);
        }
        return (root, l_ret+1);
    }

    pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // 算法1： 不符合树的结构
        // 1. 找到最深的叶节点
        // 2. 合并查找他们的父节点，在查找父节点的父节点，直到只有最后一个父节点为止。
        // 3. 返回以这个父节点即可。（函数要求）

        // 算法2： 递归
        // 1. 深度优先搜索，对树中的每个节点进行递归，返回当前子树的最大深度 d 和 lca 节点。如果当前节点为空，我们返回深度 0 和空节点。
        // 2. 比较左右子树的深度：
        //         如果左子树更深，最深叶节点在左子树中，我们返回 {左子树深度 + 1，左子树的 lca 节点}
        //         如果右子树更深，最深叶节点在右子树中，我们返回 {右子树深度 + 1，右子树的 lca 节点}
        //         如果左右子树一样深，左右子树都有最深叶节点，我们返回 {左子树深度 + 1，当前节点}
        let (l,r) = Solution::f(root);
        l
    }
}