#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[must_use]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
/// Convert binary number in a linked list to integer
///
/// # Arguments
///
/// * `head` - The head of the linked list
///
/// # Returns
///
/// * `Result<i32, &'static str>` - The integer value of the binary number
///
/// # Errors
///
/// Returns an error message "Head is None" if the input linked list is empty.
///
/// # Examples
///
/// ```
/// use rust_leetcode::encoding_decoding::get_decimal_value;
/// use rust_leetcode::encoding_decoding::ListNode;
///
/// assert_eq!(get_decimal_value(Some(Box::new(ListNode::new(1)))), Ok(1));
/// ```
pub fn get_decimal_value(head: Option<Box<ListNode>>) -> Result<i32, &'static str> {
    if head.is_none() {
        return Err("Head is None");
    }

    let mut current = head;
    let mut result = 0;

    while let Some(node) = current {
        result = (result << 1) | node.val;
        current = node.next;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let mut head = ListNode::new(1);
        let mut second = ListNode::new(0);
        let third = ListNode::new(1);

        second.next = Some(Box::new(third));
        head.next = Some(Box::new(second));

        assert_eq!(get_decimal_value(Some(Box::new(head))), Ok(5));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(get_decimal_value(None), Err("Head is None"));
    }
}
