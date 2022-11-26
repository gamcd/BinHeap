use std::cmp::Ordering;

#[derive(Copy, Clone, Eq)]
struct Node<T> {
    priority: u32,
    data: T,
}

impl<T: Eq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<T: Eq> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<T> Node<T> {
    fn new(data_: T, priority_: u32) -> Node::<T> {
        Node{
            data: data_,
            priority: priority_
        }
    }
}

#[derive(Copy, Clone)]
struct BinHeap<T> {
    arr: [Option<Node<T>>; 8192],
    size: usize
}

impl<T: Copy> BinHeap<T> {
    fn new() -> BinHeap<T> {
        BinHeap {
            arr: [None::<Node<T>>; 8192],
            size: 0
        }
    }
    
    fn enqueue(&mut self, data: T, priority: u32) -> bool {

        if self.size == 8192 {
            return false;
        }

        self.size += 1;
        self.arr[self.size] = Some(Node::new(data, priority));
        self.perc_up(self.size);
        
        true
    }

    fn perc_up(&mut self, i: usize){
        let mut cur = i;
        while cur > 1 {
            let parent = cur / 2;
            if self.arr[parent].unwrap().priority >= self.arr[cur].unwrap().priority {break}
            self.arr.swap(cur, parent);
            cur = parent;
        }
    }

    fn dequeue(&mut self) -> T {
        let rtn = self.arr[1].unwrap().data;
        self.arr[1] = self.arr[self.size];
        
        self.arr[self.size] = None;
        self.size -= 1;
        
        self.percdown(1);
        
        rtn
    }

    fn percdown(&mut self, i: usize) {

        let mut cur = i;
        while cur <= 4095 {

            let swapee: usize;
            match (self.arr[cur * 2], self.arr[cur * 2 + 1]) {
                (Some(a), Some(b)) => {
                    swapee = if a.priority < b.priority {cur * 2 + 1} else {cur * 2};
                    cur = swapee;
                },
                (Some(_), None) => swapee = cur * 2,
                (_, _) => break
            };

            if self.arr[swapee].unwrap().priority < self.arr[cur].unwrap().priority {
                self.arr.swap(cur, swapee);
            }

            cur = swapee

        }
    }
}

fn main() {
    let mut h = BinHeap::new();

    h.enqueue("G", 14);
    h.enqueue("A", 13);
    h.enqueue("V", 12);
    h.enqueue("I", 11);
    h.enqueue("N", 10);
    h.enqueue("M", 9);
    h.enqueue("C", 8);
    h.enqueue("D", 7);
    h.enqueue("I", 6);
    h.enqueue("A", 5);
    h.enqueue("R", 4);
    h.enqueue("I", 2);
    h.enqueue("M", 3);
    h.enqueue("D", 1);


    let mut f: Vec<&str> = Vec::new();

    for _ in 0..13 {
        f.push(h.dequeue());
    }

    println!("{}", f.join(""));

}
