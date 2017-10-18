pub struct Queue<'a, T: Clone + 'a> {
    v: &'a mut [T],
    tail: usize,
    head: usize
}

pub fn new<T: Clone>(v: &mut [T]) -> Queue<T> {
    Queue {
        v: v,
        head: 0,
        tail: 0
    }
}

macro_rules! incmod {
    ($x: expr, $mod: expr) => (($x + 1) % $mod);
}

impl<'a, T: Clone> Queue<'a, T> {
    pub fn enqueue(&mut self, x: T) -> Result<(), &'static str> {
        let next_tail = incmod!(self.tail, self.v.len());
        if next_tail == self.head {
            return Err("Queue overflow");
        }
        self.v[self.tail] = x.clone();
        self.tail = next_tail;
        Ok(())
    }

    pub fn dequeue(&mut self) -> Result<T, &'static str> {
        if self.head == self.tail {
            return Err("Queue underflow");
        }
        let prev_head = self.head;
        self.head = incmod!(self.head, self.v.len());
        Ok(self.v[prev_head].clone())
    }
}
