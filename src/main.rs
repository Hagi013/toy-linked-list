use std::mem::replace;
use std::fmt::Debug;

fn main() {
//    let mut ll: LinkedList<i32> = LinkedList::new();
////    ll.add(10);
////    ll.add(20);
////    ll.add(30);
////    ll.add(40);
////    ll.add(50);
//
////    println!("{:?}", ll.get_data_from_position(0));
////    println!("{:?}", ll.get_data_from_position(0));
////    println!("{:?}", ll.get_data_from_position(2));
////    println!("{:?}", ll.get_data_from_position(3));
//    let mut ll: LinkedList<i32> = LinkedList::new();
//    ll.add(2);
//    ll.add(3);
//    ll.add(10);
//    ll.change_order(3, 0);
////    assert_eq!(ll.get_data_from_position(0).unwrap(), 3);
////    assert_eq!(ll.get_data_from_position(1).unwrap(), 2);
//    ll.change_order(10, 0);
////    assert_eq!(ll.get_data_from_position(0).unwrap(), 10);
////    assert_eq!(ll.get_data_from_position(2).unwrap(), 2);
////    assert_eq!(ll.get_data_from_position(0).unwrap(), 10);
////    assert_eq!(ll.get_data_from_position(1).unwrap(), 3);
////    println!("0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3));
//
//    ll.add(25);
////    println!("0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3));
//
////    println!("{:?}", ll.get_data_from_position(1));
//    // assert_eq!(ll.get_data_from_position(3).unwrap(), 25);
//
//    let d2 = ll.get_data_from_position(ll.len() - 2).unwrap();
//    ll.change_order(ll.get_data_from_position(ll.len() - 1).unwrap(), 2);
////    println!("0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3));
//    // assert_eq!(ll.get_data_from_position(ll.len() - 1).unwrap(), d2);
//
//    ll.add(40);
//    let d3 = ll.get_data_from_position(ll.len() - 1).unwrap();
////    println!("0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4));
////    ll.change_order(ll.get_data_from_position(1).unwrap(), ll.len() - 1);
////    println!("0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4));
////    ll.change_order(ll.get_data_from_position(1).unwrap(), 2);
////    println!("0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4));
////    ll.change_order(ll.get_data_from_position(3).unwrap(), 0);
////    println!("0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4));
////    ll.change_order(ll.get_data_from_position(1).unwrap(), 4);
////    println!("0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4));
//
//    println!("0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4));
//    ll.change_order(ll.get_data_from_position(0).unwrap(), 4);
//    println!("0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4));
//    ll.change_order(ll.get_data_from_position(4).unwrap(), 0);
//    println!("0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4));
//
    let mut ll: LinkedList<i32> = LinkedList::new();
    ll.add(2);
    ll.add(3);
    ll.add(4);
    println!("0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4));
    ll.remove(2);
    println!("remove: 2, 0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4));

    ll.add(5);
    ll.add(6);
    println!("                       0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4));
    ll.change_order(ll.get_data_from_position(0).unwrap(), ll.len() - 1);
    println!("change_order: 0 -> last, 0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4));


    ll.add(20);
    println!("0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4));

    ll.add(40);
    println!("add: 40                , 0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}, 5: {:?}, 6: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4), ll.get_data_from_position(5), ll.get_data_from_position(6));
    let d6 = ll.get_data_from_position(ll.len() - 1).unwrap();
    let d7 = ll.get_data_from_position(ll.len() - 2).unwrap();
    ll.change_order(ll.get_data_from_position(ll.len() - 1).unwrap(), 0);
    println!("change_order: last -> 0, 0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}, 5: {:?}, 6: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4), ll.get_data_from_position(5), ll.get_data_from_position(6));

    ll.change_order(ll.get_data_from_position(0).unwrap(), ll.len() - 1);
    println!("change_order: 0 -> last, 0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}, 5: {:?}, 6: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4), ll.get_data_from_position(5), ll.get_data_from_position(6));

    ll.add(70);
    println!("add: 70,               0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}, 5: {:?}, 6: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4), ll.get_data_from_position(5), ll.get_data_from_position(6));

    ll.remove(5);
    println!("remove: 5            , 0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}, 5: {:?}, 6: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4), ll.get_data_from_position(5), ll.get_data_from_position(6));
    ll.change_order(ll.get_data_from_position(1).unwrap(), ll.len() - 3);
    println!("change_order: 1 -> {:?}, 0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}, 5: {:?}, 6: {:?}", ll.len() - 3, ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4), ll.get_data_from_position(5), ll.get_data_from_position(6));

    ll.remove(70);
    println!("remove: 70            , 0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}, 5: {:?}, 6: {:?}", ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4), ll.get_data_from_position(5), ll.get_data_from_position(6));
    ll.change_order(ll.get_data_from_position(ll.len() - 1).unwrap(), ll.len() - 3);
    println!("change_order: {:?} -> {:?}, 0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}, 5: {:?}, 6: {:?}", ll.len() - 1, ll.len() - 3, ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4), ll.get_data_from_position(5), ll.get_data_from_position(6));
    ll.change_order(ll.get_data_from_position(ll.len() - 1).unwrap(), ll.len() - 3);
    println!("change_order: {:?} -> {:?}, 0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}, 4: {:?}, 5: {:?}, 6: {:?}", ll.len() - 1, ll.len() - 3, ll.get_data_from_position(0), ll.get_data_from_position(1), ll.get_data_from_position(2), ll.get_data_from_position(3), ll.get_data_from_position(4), ll.get_data_from_position(5), ll.get_data_from_position(6));


    let mut ll2: LinkedList<i32> = LinkedList::new();
    ll2.add(2);
    ll2.add(3);
    println!("change_order: 0: {:?}, 1: {:?}", ll2.get_data_from_position(0), ll2.get_data_from_position(1));
}

#[test]
fn new() {
    let ll: LinkedList<i32> = LinkedList::new();
    assert_eq!(ll.len(), 0);
}

#[test]
fn add() {
    let mut ll: LinkedList<i32> = LinkedList::new();
    ll.add(2);
    assert_eq!(ll.get_data_from_position(0).unwrap(), 2);
    ll.add(3);
    assert_eq!(ll.get_data_from_position(1).unwrap(), 3);
}

#[test]
fn remove() {
    let mut ll: LinkedList<i32> = LinkedList::new();
    ll.add(2);
    ll.add(3);
    ll.add(2);
    ll.remove(2);
    assert_eq!(ll.len(), 2);
    assert_eq!(ll.get_data_from_position(0).unwrap(), 3);
    assert_eq!(ll.get_data_from_position(1).unwrap(), 2);

    ll.add(20);
    assert_eq!(ll.get_data_from_position(2).unwrap(), 20);

    ll.add(100);
    let d8 = ll.get_data_from_position(ll.len() - 1).unwrap();
    let d9 = ll.get_data_from_position(ll.len() - 2).unwrap();
    ll.change_order(ll.get_data_from_position(ll.len() - 1).unwrap(), 0);
    assert_eq!(ll.get_data_from_position(0).unwrap(), d8);
    assert_eq!(ll.get_data_from_position(ll.len() - 1).unwrap(), d9);

    ll.remove(3);
    let d10 = ll.get_data_from_position(ll.len() - 1).unwrap();
    let d11 = ll.get_data_from_position(ll.len() - 2).unwrap();
    ll.change_order(ll.get_data_from_position(ll.len() - 1).unwrap(), 0);
    assert_eq!(ll.get_data_from_position(0).unwrap(), d10);
    assert_eq!(ll.get_data_from_position(ll.len() - 1).unwrap(), d11);
}

#[test]
fn change_order() {
    let mut ll: LinkedList<i32> = LinkedList::new();
    ll.add(2);
    ll.add(3);
    ll.add(10);
    ll.change_order(3, 0);
    assert_eq!(ll.get_data_from_position(0).unwrap(), 3);
    assert_eq!(ll.get_data_from_position(1).unwrap(), 2);
    ll.change_order(10, 0);
    assert_eq!(ll.get_data_from_position(0).unwrap(), 10);
    assert_eq!(ll.get_data_from_position(2).unwrap(), 2);

    ll.add(25);
    assert_eq!(ll.get_data_from_position(3).unwrap(), 25);

    let d1 = ll.get_data_from_position(2).unwrap();
    ll.change_order(ll.get_data_from_position(1).unwrap(), 2);
    assert_eq!(ll.get_data_from_position(1).unwrap(), d1);

    let d2 = ll.get_data_from_position(ll.len() - 2).unwrap();
    ll.change_order(ll.get_data_from_position(ll.len() - 1).unwrap(), 2);
    assert_eq!(ll.get_data_from_position(ll.len() - 1).unwrap(), d2);

    ll.add(40);
    let d3 = ll.get_data_from_position(ll.len() - 1).unwrap();
    ll.change_order(ll.get_data_from_position(1).unwrap(), ll.len() - 1);
    assert_eq!(ll.get_data_from_position(ll.len() - 2).unwrap(), d3);


    let d4 = ll.get_data_from_position(0).unwrap();
    let d5 = ll.get_data_from_position(1).unwrap();
    ll.change_order(ll.get_data_from_position(0).unwrap(), 4);
    assert_eq!(ll.get_data_from_position(ll.len() - 1).unwrap(), d4);
    assert_eq!(ll.get_data_from_position(0).unwrap(), d5);

    let d6 = ll.get_data_from_position(ll.len() - 1).unwrap();
    let d7 = ll.get_data_from_position(ll.len() - 2).unwrap();
    ll.change_order(ll.get_data_from_position(4).unwrap(), 0);
    assert_eq!(ll.get_data_from_position(0).unwrap(), d6);
    assert_eq!(ll.get_data_from_position(ll.len() - 1).unwrap(), d7);
}


use std::boxed;
#[derive(Copy, Clone, Debug)]
struct LinkedListNode<T>
    where
        T: Copy,
        T: PartialEq,
        T: Debug,
{
    prev: Option<*mut LinkedListNode<T>>,
    next: Option<*mut LinkedListNode<T>>,
    data: T,
}

impl<T> LinkedListNode<T>
    where
        T: Copy,
        T: PartialEq,
        T: Debug,
{
    pub fn new(data: T) -> Self {
        LinkedListNode {
            prev: None,
            next: None,
            data,
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T>
    where
        T: Copy,
        T: PartialEq,
        T: Debug,
{
    head: Option<*mut LinkedListNode<T>>,
    tail: Option<*mut LinkedListNode<T>>,
    count: usize,
}

impl<T> LinkedList<T>
    where
        T: Copy,
        T: PartialEq,
        T: Debug,
{
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            count: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn push_front(&mut self, data: T) -> Result<(), String> {
        unsafe {
            let node: *mut LinkedListNode<T> = boxed::Box::into_raw(boxed::Box::new(LinkedListNode::new(data)));
            if self.len() == 0 {
                self.head = Some(node);
                self.tail = Some(node);
                (*node).prev = None;
                (*node).next = None;
            } else {
                let head = self.head.ok_or("LinkedList's head is none.".to_string())?;
                self.head = Some(node);
                (*head).prev = Some(node);
                (*node).next = Some(head);
                (*node).prev = None;
            }
            self.count += 1;
            return Ok(());
        }
    }

    // add???????????????????????????????????????
    pub fn add(&mut self, data: T) -> Result<(), String> {
        unsafe {
            let node: *mut LinkedListNode<T> = boxed::Box::into_raw(boxed::Box::new(LinkedListNode::new(data)));
            if self.len() == 0 {
                self.head = Some(node);
                self.tail = Some(node);
                (*node).prev = None;
                (*node).next = None;
            } else {
                let tail = self.tail.ok_or("LinkedList's tail is none.".to_string())?;
                self.tail = Some(node);
                (*tail).next = Some(node);
                (*node).prev = Some(tail);
                (*node).next = None;
            }
            self.count += 1;
            return Ok(());
        }
    }

    pub fn remove(&mut self, data: T) -> Result<(), String> {
        if self.len() == 0 {
            return Err("LinkedList length is 0.".to_string());
        }

        if self.get_position_from_data(data).is_none() {
            return Err("Data is not existing.".to_string());
        }

        self.count -= 1;
        // ????????????????????????????????????????????????
        if self.count == 0 {
            self.head = None;
            self.tail = None;
            return Ok(());
        }

        unsafe {
            let pointer: *mut LinkedListNode<T> = self.get_pointer_from_data(data).ok_or("data is not existing in LinkedList.".to_string())?;

            if self.head.is_some() && self.tail.is_some() { // ?????????????????????????????????????????????head???tail?????????????????????
                let head: *mut LinkedListNode<T> = self.head.unwrap();
                let tail: *mut LinkedListNode<T> = self.tail.unwrap();

                if pointer != head && pointer != tail { // head???tail???????????????????????????????????????????????????
                    if let Some(prev) = (*pointer).prev {
                        replace(&mut (*prev).next, (*pointer).next);
                    }
                    if let Some(next) = (*pointer).next {
                        replace(&mut (*next).prev, (*pointer).prev);
                    }
                    (*pointer).prev = None;
                    (*pointer).next = None;
                } else if pointer == head { // head????????????????????????
                    replace(&mut self.head, replace(&mut (*pointer).next, None));
                    replace(&mut (*self.head.unwrap()).prev, None);
                } else if pointer == tail { // tail????????????????????????
                    replace(&mut self.tail, replace(&mut (*pointer).prev, None));
                    replace(&mut (*self.tail.unwrap()).next, None);
                } else {
                    self.head = None;
                    self.tail = None;
                }
                return Ok(());
            } else {
                return Err("Element in LinkedList is null.".to_string());
            }
        }
    }

    pub fn change_order(&mut self, data: T, idx: usize) -> Result<(), String> {
        let src_node_ptr: *mut LinkedListNode<T> = self.get_pointer_from_data(data).ok_or("data is not existing in LinkedList".to_string())?;
        let position = self.get_position_from_data(data).ok_or("In chane_order, data's position is not found.".to_string())?;
        // println!("{:?}", unsafe { (&mut *src_node_ptr) });
        let order: usize =
            if idx >= self.len() { self.len() - 1 }
            else { idx };

        unsafe {
            // ?????????????????????Node????????????????????????????????????
            if let Some(prev) = (*src_node_ptr).prev {
                replace(&mut (*prev).next,(*src_node_ptr).next);
            }
            if let Some(next) = (*src_node_ptr).next {
                replace(&mut (*next).prev, (*src_node_ptr).prev);
            }
            if order == 0 { // head???????????????
                if let Some(head_node_ptr) = self.head {
                    if position == self.len() - 1 { // ?????????????????????????????????Node??????????????????tail???????????????
                        self.tail = (*src_node_ptr).prev;
                    }
                    (*head_node_ptr).prev = Some(src_node_ptr);
                    (*src_node_ptr).next = Some(head_node_ptr);
                    (*src_node_ptr).prev = None;
                    self.head = Some(src_node_ptr);
                    return Ok(());
                } else {
                    // ??????head??????????????????????????????????????????(?????????????????????????????????????????????)
                    return self.add(data);
                }
            } else if order == self.len() - 1 { // tail???????????????
                if position == 0 { // ?????????????????????????????????Node??????????????????head???????????????
                    self.head = (*src_node_ptr).next;
                }
                let tail_node_ptr: *mut LinkedListNode<T> = self.tail.ok_or("LinkedList is broken.".to_string())?;
                (*tail_node_ptr).next = Some(src_node_ptr);
                (*src_node_ptr).prev = Some(tail_node_ptr);
                (*src_node_ptr).next = None;
                self.tail = Some(src_node_ptr);
                return Ok(());
            } else {
                // ???????????????????????????????????????????????????
                // 1. ????????????????????????Node????????????????????????Node?????????
                let dest_order_node_ptr: *mut LinkedListNode<T> = self.get_pointer_from_index(order)
                    .ok_or("idx argument to change_order may be out of bound.".to_string())?;
                let dest_order_prev_node_ptr = (*dest_order_node_ptr).prev
                    .ok_or("change_order target order node's state is broken in LinkedList.".to_string())?;

                // 2. ??????????????????Node?????????????????????
                (*dest_order_node_ptr).prev = Some(src_node_ptr);
                (*dest_order_prev_node_ptr).next = Some(src_node_ptr);

                // 3. ??????????????????Node??????????????????head????????????tail????????????????????????
                if position == self.len() - 1 { // ?????????????????????????????????Node??????????????????tail???????????????
                    self.tail = (*src_node_ptr).prev;
                } else if position == 0 { // ?????????????????????????????????Node??????????????????head???????????????
                    self.head = (*src_node_ptr).next;
                }

                // 4. ??????????????????Node??????????????????????????????
                (*src_node_ptr).next = Some(dest_order_node_ptr);
                (*src_node_ptr).prev = Some(dest_order_prev_node_ptr);
                return Ok(());
            }
        }
    }

    pub fn get_position_from_data(&self, data: T) -> Option<usize> {
        if self.len() == 0 { return None; }
        let mut idx: usize = 0;
        let mut node: *mut LinkedListNode<T> = self.head.or(None)?;
        loop {
            unsafe {
                if data == (*node).data {
                    return Some(idx);
                }
                if let Some(next) = (*node).next {
                    idx += 1;
                    node = next;
                } else {
                    break;
                }
            }
        }
        return None;
    }

    pub fn get_data_from_position(&self, idx: usize) -> Option<T> {
        if idx >= self.count {
            return None;
        }
        let data_ptr: *mut LinkedListNode<T> = self.get_pointer_from_index(idx).or(None)?;
        return unsafe { Some((*data_ptr).data) };
    }

    pub fn get_next_data(&self, data: T) -> Option<T> {
        match self.get_position_from_data(data) {
            Some(idx) => {
                if idx + 1 >= self.len() {
                    return None;
                }
                let next_data_ptr: *mut LinkedListNode<T> = self.get_pointer_from_index(idx + 1).or(None)?;
                return unsafe { Some((*next_data_ptr).data) };
            },
            None => None,
        }
    }

    pub fn get_prev_data(&self, data: T) -> Option<T> {
        match self.get_position_from_data(data) {
            Some(idx) => {
                if idx <= 0 { return None; }
                let prev_data_ptr: *mut LinkedListNode<T> = self.get_pointer_from_index(idx - 1).or(None)?;
                return unsafe { Some((*prev_data_ptr).data) };
            },
            None => None,
        }
    }

    fn get_pointer_from_data(&self, data: T) -> Option<*mut LinkedListNode<T>> {
        if self.len() == 0 { return None }
        let mut node: *mut LinkedListNode<T>  = self.head.or(None)?;
        loop {
            unsafe {
                if data == (*node).data {
                    return Some(node);
                }
                if let Some(next) = (*node).next {
                    node = next;
                } else {
                    break;
                }
            }
        }
        return None;
    }

    fn get_pointer_from_index(&self, idx: usize) -> Option<*mut LinkedListNode<T>> {
        if idx >= self.len() { return None; }
        let mut node: *mut LinkedListNode<T> = self.head.or(None)?;

        for _i in 0..idx {
            unsafe {
                if let Some(next) = (*node).next {
                    node = next;
                } else {
                    return None;
                }
            }
        }
        return Some(node)
    }
}

//#[cfg(test)]
//mod test {
//    use self::util::linked_list::LinkedList;
//
//    #[test]
//    fn new() {
//        let ll: LinkedList<i32> = LinkedList::new();
//        assert_eq!(ll.len(), 0);
//    }
//}
