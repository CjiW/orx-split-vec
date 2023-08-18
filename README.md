A split vector is a vector represented as a sequence of
multiple contagious data fragments.

It provides the following features:

* Flexible in growth strategies; custom strategies can be defined.
* Growth does not cause any memory copies.
* Capacity of an already created fragment is never changed.
* The above feature allows the data to stay pinned in place. Memory location of an item added to the split vector will never change unless it is removed from the vector or the vector is dropped.

# Why - Actually

The main feature of `SplitVec` is that it guarantees that the memory locations of its elements
will never change.

Together with rust's ownership model, this turns out to be a useful property
and makes `SplitVec` the underlying model of other useful data structures.

See for instance [orx-imp-vec](https://crates.io/crates/orx-imp-vec).


# Why - Also

`SplitVec` is certainly not a replacement for `Vec`,
and not preferable over it most of the cases as it adds one level of abstraction.

It is useful for building a collection where:

* contagious layout of the entire collection is not critical since we are more often pushing to the vector than reading elements,
* so that the copies while building a vector due to uncertainty in capacity can be avoided.

SplitVec provides a std::vec::Vec like api for convenience and makes it easy to convert between these types.

```rust
use orx_split_vec::{FragmentGrowth, SplitVec};

// the capacity will be expanded in chunks of 10 items
// see 'FragmentGrowth::exponential' and 'FragmentGrowth::by_function' for alternative flexible growth strategies.
let growth = FragmentGrowth::constant(10);
let mut split_vec = SplitVec::with_growth(growth);

// below insertions will lead to 7 expansions, creating 7 vectors with capacity of 10.
// no memory copies will happen during the building.
for i in 0..70 {
    split_vec.push(i);
}

// this vector can be used as a split vector due to its standar vector like api.
// alternatively, it can be collected into a vec with a contagious layout once build-up is complete.

let vec: Vec<_> = split_vec.as_vec();
```
