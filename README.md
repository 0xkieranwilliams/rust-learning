# Rust by Example Checklist

## Checklist to track your progress as you work through the material of rust by example.

[Rust by Example](https://doc.rust-lang.org/rust-by-example/) 

---

## **Introduction**
- [x] Introduction

## **1. Hello World**
- [x] Hello World
- [x] Comments
- [x] Formatted print
  - [x] Debug
  - [x] Display
  - [x] Formatting

## **2. Primitives**
- [x] Literals and operators
- [x] Tuples
- [x] Arrays and Slices

## **3. Custom Types**
- [x] Structures
- [x] Enums
- [x] Constants

## **4. Variable Bindings**
- [x] Mutability
- [x] Scope and Shadowing
- [x] Declare first
- [x] Freezing

## **5. Types**
- [x] Casting
- [x] Literals
- [x] Inference
- [x] Aliasing

## **6. Conversion**
- [x] From and Into
- [x] TryFrom and TryInto
- [x] To and from Strings

## **7. Expressions**
- [x] Expressions

## **8. Flow of Control**
- [x] if/else
- [x] loop
  - [x] Nesting and labels
  - [x] Returning from loops
- [x] while
- [x] for and range
- [x] match
  - [x] Destructuring
  - [x] Guards
  - [x] Binding
- [x] if let
- [x] let-else
- [x] while let

## **9. Functions**
- [ ] Methods
- [ ] Closures
  - [ ] Capturing
  - [ ] As input parameters
  - [ ] Type anonymity
  - [ ] Input functions
  - [ ] As output parameters
  - [ ] Examples in std
    - [ ] Iterator::any
    - [ ] Searching through iterators
- [ ] Higher Order Functions
- [ ] Diverging functions

## **10. Modules**
- [ ] Visibility
- [ ] Struct visibility
- [ ] The use declaration
- [ ] super and self
- [ ] File hierarchy

## **11. Crates**
- [ ] Creating a Library
- [ ] Using a Library

## **12. Cargo**
- [ ] Dependencies
- [ ] Conventions
- [ ] Tests
- [ ] Build Scripts

## **13. Attributes**
- [ ] dead_code
- [ ] Crates
- [ ] cfg
  - [ ] Custom

## **14. Generics**
- [ ] Functions
- [ ] Implementation
- [ ] Traits
- [ ] Bounds
  - [ ] Testcase: empty bounds
- [ ] Multiple bounds
- [ ] Where clauses
- [ ] New Type Idiom
- [ ] Associated items
  - [ ] The Problem
  - [ ] Associated types
- [ ] Phantom type parameters
  - [ ] Testcase: unit clarification

## **15. Scoping Rules**
- [ ] RAII
- [ ] Ownership and moves
  - [ ] Mutability
  - [ ] Partial moves
- [ ] Borrowing
  - [ ] Mutability
  - [ ] Aliasing
  - [ ] The ref pattern
- [ ] Lifetimes
  - [ ] Explicit annotation
  - [ ] Functions
  - [ ] Methods
  - [ ] Structs
  - [ ] Traits
  - [ ] Bounds
  - [ ] Coercion
  - [ ] Static
  - [ ] Elision

## **16. Traits**
- [ ] Derive
- [ ] Returning Traits with dyn
- [ ] Operator Overloading
- [ ] Drop
- [ ] Iterators
- [ ] impl Trait
- [ ] Clone
- [ ] Supertraits
- [ ] Disambiguating overlapping traits

## **17. macro_rules!**
- [ ] Syntax
  - [ ] Designators
  - [ ] Overload
  - [ ] Repeat
- [ ] DRY (Don't Repeat Yourself)
- [ ] DSL (Domain Specific Languages)
- [ ] Variadics

## **18. Error Handling**
- [ ] panic
- [ ] abort & unwind
- [ ] Option & unwrap
  - [ ] Unpacking options with ?
  - [ ] Combinators: map
  - [ ] Combinators: and_then
  - [ ] Defaults: or, or_else, get_or_insert, get_or_insert_with
- [ ] Result
  - [ ] map for Result
  - [ ] aliases for Result
  - [ ] Early returns
  - [ ] Introducing ?
- [ ] Multiple error types
  - [ ] Pulling Results out of Options
  - [ ] Defining an error type
  - [ ] Boxing errors
  - [ ] Other uses of ?
  - [ ] Wrapping errors
- [ ] Iterating over Results

## **19. Std Library Types**
- [ ] Box, stack, and heap
- [ ] Vectors
- [ ] Strings
- [ ] Option
- [ ] Result
  - [ ] ?
- [ ] panic!
- [ ] HashMap
  - [ ] Alternate/custom key types
  - [ ] HashSet
- [ ] Rc
- [ ] Arc

## **20. Std Misc**
- [ ] Threads
  - [ ] Testcase: map-reduce
- [ ] Channels
- [ ] Path
- [ ] File I/O
  - [ ] open
  - [ ] create
  - [ ] read_lines
- [ ] Child processes
  - [ ] Pipes
  - [ ] Wait
- [ ] Filesystem Operations
- [ ] Program arguments
  - [ ] Argument parsing
- [ ] Foreign Function Interface

## **21. Testing**
- [ ] Unit testing
- [ ] Documentation testing
- [ ] Integration testing
- [ ] Dev-dependencies

## **22. Unsafe Operations**
- [ ] Inline assembly

## **23. Compatibility**
- [ ] Raw identifiers

## **24. Meta**
- [ ] Documentation
- [ ] Playground

---

### **How to Use**
1. Copy the relevant example from the book into `src/main.rs` or create a new binary in `src/bin`.
2. Check off items as you complete them to track your progress.
3. Experiment with variations of each example to deepen your understanding.

