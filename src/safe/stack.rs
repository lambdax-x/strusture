pub struct Stack<'a, T: Clone + 'a> {
    v: &'a mut [T],
    top: usize
}

impl<'a, T: Clone> Stack<'a, T> {
    pub fn new(v: &'a mut [T]) -> Self {
        Stack {
            v: v,
            top: 0
        }
    }

    pub fn new_with(v: &'a mut [T], top: usize) -> Result<Self, ()> {
        if !(top < v.len()) {
            return Err(());
        }
        let stack = Stack {
            v: v,
            top: top
        };
        Ok(stack)
    }

    pub fn push(&mut self, x: T) -> Result<(), &'static str> {
        if self.top == self.v.len() - 1 {
            return Err("stack is full");
        }
        self.v[self.top] = x.clone();
        self.top += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        Some(self.v[self.top].clone())
    }

    pub fn empty(&self) -> bool {
        self.top == 0
    }
}
