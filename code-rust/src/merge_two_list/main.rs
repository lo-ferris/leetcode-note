/// 合并两个有序链表
/// 将两个升序链表合并为一个新的升序链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。

/// 示例
/// 输入: 1->2->4, 1->3->4
/// 输出: 1->1->2->3->4->4

#[derive(PartialEq, Eq, Clone, code_rust_macro::ListNodeDebug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/*impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut node = &mut self.clone();
        let mut str = "".to_string();
        loop {
            str.push_str(&node.val.to_string());
            str.push_str("->");
            node = match node.next.as_mut() {
                Some(n) => n.as_mut(),
                None => break,
            };
        }
        str.remove(str.len() - 1);
        str.remove(str.len() - 1);
        write!(f, "{}", str)
    }
}*/

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut new_list_node = Some(Box::new(ListNode::new(0)));
        let mut prev_new_node = &mut new_list_node;
        let (mut node1, mut node2) = (&l1, &l2);
        while node1.is_some() && node2.is_some() {
            let node_box1 = node1.as_ref().unwrap();
            let node_box2 = node2.as_ref().unwrap();
            // 判断左边链表节点是否小于等于右边链表节点
            if node_box1.val <= node_box2.val {
                // 将下一个节点添加到上一个节点的后面
                prev_new_node.as_mut().unwrap().next =
                    Some(Box::new(ListNode::new(node_box1.val)));
                // 左边当前链表节点后移
                node1 = &node_box1.next;
            } else {
                // 判断左边链表节点是大于右边链表节点
                // 将下一个节点添加到上一个节点的后面
                prev_new_node.as_mut().unwrap().next =
                    Some(Box::new(ListNode::new(node_box2.val)));
                // 右边当前链表节点后移
                node2 = &node_box2.next;
            }
            // 记录上一个节点
            prev_new_node = &mut prev_new_node.as_mut().unwrap().next;
        }

        // 左边链表还有剩余节点，直接添加到末尾
        if node1.is_some() {
            prev_new_node.as_mut().unwrap().next = node1.clone()
        }
        // 右边链表还有剩余节点，直接添加到末尾
        if node2.is_some() {
            prev_new_node.as_mut().unwrap().next = node2.clone();
        }

        new_list_node.take().unwrap().next
    }
}

#[test]
fn test_merge_two_lists() {
    let mut l1 = Some(Box::new(ListNode::new(1)));
    let mut node1 = Some(Box::new(ListNode::new(2)));
    let node2 = Some(Box::new(ListNode::new(4)));

    node1.as_mut().unwrap().next = node2;
    l1.as_mut().unwrap().next = node1;

    let mut l2 = Some(Box::new(ListNode::new(1)));
    let mut node3 = Some(Box::new(ListNode::new(3)));
    let node4 = Some(Box::new(ListNode::new(4)));

    node3.as_mut().unwrap().next = node4;
    l2.as_mut().unwrap().next = node3;

    println!("{:?}", l1);
    println!("{:?}", l2);

    let new_list_node = Solution::merge_two_lists(l1, l2);
    println!("{:?}", new_list_node);
}
