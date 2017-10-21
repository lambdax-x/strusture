macro_rules! get_id {
    ($i: expr, parent) => (($i - 1) / 2);
    ($i: expr, left) => ($i * 2 + 1);
    ($i: expr, right) => ($i * 2 + 2);
}

macro_rules! gen_build_heap {
    ($self: ident) => {
        for i in (0 .. $self.size / 2).rev() {
            $self.heapify(i);
        }
    }
}

macro_rules! gen_heapify {
    ($self: ident, $i: ident, $policy: expr) => {
        let mut largest = $i;
        let left = get_id!($i, left);
        let right = get_id!($i, right);

        if left <= $self.size && $policy(&$self.v[left], &$self.v[$i]) {
            largest = left;
        }

        if right <= $self.size && $policy(&$self.v[right], &$self.v[largest]) {
            largest = right;
        }

        if largest != $i {
            $self.v.swap(largest, $i);
            $self.heapify(largest);
        }
    }
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
     $build: ident,
     $policy: expr,
     $top: ident,
     $extract: ident,
     $update_key: ident
    ) => {
       impl<'a, T: Clone + Ord> $name<'a, T> {
            pub fn new(v: &'a mut [T]) -> Self {
                $name {
                    v: v,
                    size: 0
                }
            }

            /// Returns `true` if the $name contains no elements
            pub fn empty(&self) -> bool {
                self.size == 0
            }

            pub fn size(&self) -> usize {
                self.size
            }

            pub fn $build(&mut self) {
                gen_build_heap!(self);
            }

            fn heapify(&mut self, i: usize) {
                gen_heapify!(self, i, $policy);
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

            fn sift_up(&mut self, i: usize) {
                let mut i = i;
                while i > 0 && $policy(&self.v[i], &self.v[get_id!(i, parent)]) {
                    self.v.swap(i, get_id!(i, parent));
                    i = get_id!(i, parent);
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
     $build: ident,
     $policy: expr,
     $top: ident,
     $extract: ident,
     $update_key: ident
    ) => {
        gen_heap_struct!($name);
        gen_heap_methods!(
            $name,
            $build,
            $policy,
            $top,
            $extract,
            $update_key
        );
    }
}
