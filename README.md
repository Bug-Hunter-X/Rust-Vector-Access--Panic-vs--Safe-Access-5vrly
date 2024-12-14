# Rust Vector Access: Avoiding Panics

This example demonstrates a common error in Rust when working with vectors: accessing elements without checking if the vector is empty.  Direct indexing (`[]`) will cause a panic if the index is out of bounds.  The solution shows how to use the `get()` method for safe access.

## Bug

The `bug.rs` file shows how attempting to access the first element of an empty vector leads to a panic.

## Solution

The `bugSolution.rs` file demonstrates using `get()`, which returns an `Option`, allowing for safe handling of potentially empty vectors.