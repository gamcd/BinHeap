#[derive(Copy, Clone)]
struct Node<T> {
    priority: u32,
    data: T,
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
    
    fn enqueue(&mut self, data: T, priority: u32) {
        self.arr[self.size] = Some(Node::new(data, priority));
        
        let mut current: usize = self.size;
        let mut swap = self.arr[current];
        
        while current != 1 {
            if self.arr[current / 2].unwrap().priority < priority {
               current /= 2;
            } else {
                self.size += 1;
                std::mem::swap(&mut swap, &mut self.arr[current]);
                break;
            }
        }
        
    }
    fn dequeue(&mut self) -> T {
        let rtn = self.arr[1].unwrap().data;
        self.arr[1] = self.arr[self.size];
        
        self.arr[self.size] = None;
        self.size -= 1;
        
        let mut current = 1;
        while current < self.size / 2 {
            let child1 = self.arr[current * 2];
            let child2 = self.arr[current * 2 + 1];
            
            current *= 2;
        }
        
        rtn
    }
}

fn main() {
    
}
