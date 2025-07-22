use std::rc::Rc;

pub struct LinkedList<T> {
    len: usize,
    head: Option<Rc<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    pub fn insert_ahead(&mut self, data: T) {
        let new_node = Node {
            data,
            next: self.next.clone(),
        };

        self.next = Some(Rc::new(new_node))
    }

    pub fn delete_ahead(&mut self) {
        if let Some(next_node) = &self.next {
            self.next = next_node.next.clone()
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { len: 0, head: None }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn push(&mut self, data: T) {
        let new_node = Node {
            data,
            next: self.head.clone(),
        };

        self.head = Some(Rc::new(new_node));
        self.len += 1;
    }

    pub fn pop(&mut self) {
        if let Some(entry) = &self.head {
            self.head = entry.next.clone();
            self.len -= 1;
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: &self.head,
        }
    }

    pub fn contains(&self, element: T) -> bool
    where
        T: PartialEq,
    {
        self.iter().any(|item| item == &element)
    }

    pub fn insert(&mut self, pos: usize, data: T) {
        if pos == 0 {
            return self.push(data);
        }

        if pos > self.len {
            return;
        }

        let left_node = self.find_node_as_mut(pos - 1);

        if let Some(node) = left_node {
            node.insert_ahead(data);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, pos: usize) {
        if pos == 0 {
            return self.pop();
        }

        if pos >= self.len {
            return;
        }

        let left_node = self.find_node_as_mut(pos - 1);

        if let Some(node) = left_node {
            node.delete_ahead();
            self.len -= 1;
        }
    }

    fn find_node_as_mut(&mut self, pos: usize) -> Option<&mut Node<T>> {
        if pos >= self.len {
            return None;
        }

        let mut i = pos;
        let mut current = &mut self.head;

        while i > 0 {
            let rc = current.as_mut()?;

            current = &mut Rc::get_mut(rc)?.next;

            i -= 1;
        }

        Rc::get_mut(current.as_mut()?)
    }

    pub fn reverse(&self) -> LinkedList<T>
    where
        T: Clone,
    {
        let mut list = LinkedList::new();
        self.iter().for_each(|data| list.push(data.clone()));
        list
    }
}

pub struct Iter<'a, T> {
    current: &'a Option<Rc<Node<T>>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.as_ref();

        if let Some(node) = current {
            self.current = &node.next;
        }

        current.map(|node| &node.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut linked_list: LinkedList<i32> = LinkedList::new();
        assert_eq!(linked_list.len(), 0);
        assert_eq!(linked_list.head(), None);

        linked_list.push(1);
        assert_eq!(linked_list.len(), 1);
        assert_eq!(linked_list.head(), Some(&1));
    }
}

// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }

//     fn from_vec(nums: &[i32]) -> Option<Box<ListNode>> {
//         let mut result = None;
//         for i in nums.iter().rev() {
//             let mut node = ListNode::new(*i);
//             node.next = result;
//             result = Some(Box::new(node));
//         }

//         result
//     }
// }

// pub fn merge_two_lists(
//     list1: Option<Box<ListNode>>,
//     list2: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
//     let mut output = None;
//     loop {}
//     output
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn ex1() {
//         let list_1 = ListNode::from_vec(&vec![1, 2, 4]);
//         let list_2 = ListNode::from_vec(&vec![1, 3, 4]);
//         let merged_list = merge_two_lists(list_1, list_2);

//         let mut index = 0;
//         let expected = vec![1, 2, 3, 4];
//         while let Some(entry) = merged_list.iter().next() {
//             assert_eq!(entry.val, expected[index]);
//             index += 1;
//         }

//         assert_eq!(index, expected.len())
//     }

//     #[test]
//     fn ex2() {
//         let list_1 = ListNode::from_vec(&vec![1, 2, 4]);
//         for value in list_1.iter() {}

//         let mut index = 0;
//         let expected = vec![1, 2, 4];

//         assert_eq!(index, expected.len())
//     }
// }
