#[macro_use]
mod macros;

gen_heap!(
    MaxHeap,
    |a, b| a > b,
    max,
    extract_max,
    increase_key
);

gen_heap!(
    MinHeap,
    |a, b| a < b,
    min,
    extract_min,
    decrease_key
);
