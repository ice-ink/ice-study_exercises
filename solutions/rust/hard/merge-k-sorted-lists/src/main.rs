#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn merge_two_lists(
    mut a: Option<Box<ListNode>>,
    mut b: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut tail = &mut head;

    loop {
        match (a, b) {
            (Some(x), Some(y)) => {
                if x.val <= y.val {
                    let mut x = x;
                    let rest = x.next.take();
                    *tail = Some(x);
                    a = rest;
                    b = Some(y);
                } else {
                    let mut y = y;
                    let rest = y.next.take();
                    *tail = Some(y);
                    a = Some(x);
                    b = rest;
                }
                tail = match tail {
                    Some(node) => &mut node.next,
                    None => unreachable!(),
                };
            }
            (None, rest) | (rest, None) => {
                *tail = rest;
                break;
            }
        }
    }

    head
}

pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.is_empty() {
        return None;
    }

    // Repeatedly merge pairs (divide and conquer) until one list remains.
    let mut interval = 1usize;
    while interval < lists.len() {
        let mut i = 0usize;
        while i + interval < lists.len() {
            let left = lists[i].take();
            let right = lists[i + interval].take();
            lists[i] = merge_two_lists(left, right);
            i += interval * 2;
        }
        interval *= 2;
    }

    lists[0].take()
}

fn list_from(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &v in values.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

fn main() {
    let lists = vec![
        list_from(&[1, 4, 5]),
        list_from(&[1, 3, 4]),
        list_from(&[2, 6]),
    ];
    println!("{:?}", list_to_vec(merge_k_lists(lists)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let lists = vec![
            list_from(&[1, 4, 5]),
            list_from(&[1, 3, 4]),
            list_from(&[2, 6]),
        ];
        assert_eq!(
            list_to_vec(merge_k_lists(lists)),
            vec![1, 1, 2, 3, 4, 4, 5, 6]
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(merge_k_lists(vec![]), None);
    }

    #[test]
    fn single_list() {
        let lists = vec![list_from(&[1, 2, 3])];
        assert_eq!(list_to_vec(merge_k_lists(lists)), vec![1, 2, 3]);
    }

    #[test]
    fn some_empty_lists() {
        let lists = vec![
            list_from(&[]),
            list_from(&[1, 2]),
            list_from(&[]),
            list_from(&[3]),
        ];
        assert_eq!(list_to_vec(merge_k_lists(lists)), vec![1, 2, 3]);
    }
}
