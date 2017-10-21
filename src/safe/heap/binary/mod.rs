#[macro_use]
mod macros;

gen_heap!(
    MaxHeap,
    build_max_heap,
    |a, b| a > b,
    max,
    extract_max,
    increase_key
);

gen_heap!(
    MinHeap,
    build_min_heap,
    |a, b| a < b,
    min,
    extract_min,
    decrease_key
);
