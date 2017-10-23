macro_rules! get_node {
    ($i: expr, Parent) => (($i - 1) / 2);
    ($i: expr, Left) => ($i * 2 + 1);
    ($i: expr, Right) => ($i * 2 + 2);
}

macro_rules! gen_heap_struct {
    ($name: ident) => {
        pub struct $name<'a, T: Clone + Ord + 'a> {
            v: &'a mut [T],
            size: usize
        } 
    }
}

macro_rules! gen_heap_methods {
    ($name: ident,
     $policy: expr,
     $top: ident,
     $extract: ident,
     $update_key: ident
    ) => {
       impl<'a, T: Clone + Ord> $name<'a, T> {
           /// Constructs a new empty heap
           pub fn new(v: &'a mut [T]) -> Self {
               $name {
                   v: v,
                   size: 0
               }
           }

           /// Constructs a new heap with the `size` number of elements already
           /// present in the array
           pub fn new_with(v: &'a mut [T], size: usize) -> Result<Self, ()> {
               if size > v.len() {
                   return Err(());
               }
               let mut heap = $name {
                   v: v,
                   size: size
               };
               heap.build();
               Ok(heap)
           }

           /// Returns `true` if the $name contains no elements
           pub fn empty(&self) -> bool {
               self.size == 0
           }

           /// Returns the number of elements in the heap
           pub fn size(&self) -> usize {
               self.size
           }

           /// Builds the heap, called once when created 
           fn build(&mut self) {
               for node in (0 .. self.size / 2).rev() {
                   self.heapify(node);
               }
           }

           fn heapify(&mut self, node: usize) {
               let mut new = node;
               let left = get_node!(node, Left);
               let right = get_node!(node, Right);

               // find the node which should be root of this sub-heap
               if left <= self.size && $policy(&self.v[left], &self.v[node]) {
                   new = left;
               }
               if right <= self.size && $policy(&self.v[right], &self.v[new]) {
                   new = right;
               }

               // Set the new root and heapify down if a new root has been found
               if new != node {
                   self.v.swap(new, node);
                   self.heapify(new);
               }
           }
        
           fn sift_up(&mut self, node: usize) {
               if node == 0 {
                   return;
               }

               let mut node = node;
               while node > 0  {
                   let parent = get_node!(node, Parent);
                   if !$policy(&self.v[node], &self.v[parent]) {
                       break;
                   }
                   self.v.swap(node, parent);
                   node = parent;
               }
           }

           pub fn $top(&self) -> Option<T> {
               match self.size {
                   0 => None,
                   _ => Some(self.v[0].clone())
               }
           }

           pub fn $extract(&mut self) -> Option<T> {
               match self.size {
                   0 => None,
                   1 => {
                       self.size = 0;
                       Some(self.v[0].clone())
                   },
                   size => {
                       let top = self.v[0].clone();
                       let last = self.v[size - 1].clone();
                       self.v[0] = last;
                       self.size = size - 1;
                       self.heapify(0);
                       Some(top)
                   }
               }
           }

           pub fn $update_key(&mut self, i: usize, key: T) -> Result<(), ()> {
               if !($policy(&key, &self.v[i]) || key == self.v[i]) {
                   return Err(());
               }
               self.v[i] = key;
               self.sift_up(i);
               Ok(())
           }

           pub fn insert(&mut self, key: T) -> Result<(), ()> {
               let size = self.size;
               if size == self.v.len() {
                   return Err(());
               }

               self.v[size] = key.clone();
               self.sift_up(size);
               self.size += 1;
               Ok(())
           }
       }
    }
}

macro_rules! gen_heap {
    ($name: ident,
     $policy: expr,
     $top: ident,
     $extract: ident,
     $update_key: ident
    ) => {
        gen_heap_struct!($name);
        gen_heap_methods!(
            $name,
            $policy,
            $top,
            $extract,
            $update_key
        );
    }
}
