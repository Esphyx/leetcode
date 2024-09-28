use super::{ListNode, Solution};

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
#[test]
pub fn test() {
    fn generate(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut next = None;

        for &value in values.iter().rev() {
            let mut previous = Box::new(ListNode::new(value));
            previous.next = next;
            next = Some(previous);
        }

        next
    }

    assert_eq!(
        Solution::add_two_numbers(generate(vec![2, 4, 3]), generate(vec![5, 6, 3])),
        generate(vec![7, 0, 8])
    );

    assert_eq!(
        Solution::add_two_numbers(generate(vec![0]), generate(vec![0])),
        generate(vec![0])
    );

    assert_eq!(
        Solution::add_two_numbers(
            generate(vec![9, 9, 9, 9, 9, 9, 9]),
            generate(vec![9, 9, 9, 9])
        ),
        generate(vec![8, 9, 9, 9, 0, 0, 0, 1])
    );
}
