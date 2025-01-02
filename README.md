# Rust Undefined Behavior Example

This repository demonstrates a common source of undefined behavior in Rust: modifying a vector through a raw pointer after the vector's internal structure has been changed. This typically happens when the vector's length or capacity is modified after obtaining the raw pointer.  The program may appear to work correctly in some cases, but is not guaranteed to behave consistently across different compiler versions, optimization levels, or even runs.  The solution focuses on safe and idiomatic Rust practices.

## Bug

The `bug.rs` file contains code that demonstrates the problem.  It gets a raw pointer to the vector's underlying data and then attempts to modify this memory location after the vector has been potentially resized, leading to undefined behavior.  

## Solution

The `bugSolution.rs` file provides a corrected version. This version demonstrates safe memory management and avoids the risk of undefined behavior by utilizing safe Rust patterns.