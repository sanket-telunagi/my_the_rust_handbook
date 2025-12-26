fn main() {
    println!("Hello, world!");

    // memory management overall
    /*
        1. Garbage Collection
            - Pros: Easier for developers, automatic memory management, error free* (mostly), faster write time
            - Cons: Performance overhead, unpredictable pauses, no control over memory, larger program size

        2. Manual Memory Management
            - Pros: Full control over memory, potentially better performance, predictable behavior, smaller program size
            - Cons: More complex code, higher chance of memory leaks and bugs, longer development time, error prone, slower write time

        3. Ownership (Rust's approach)
            - Pros: Memory safety without garbage collection, predictable performance, automatic memory management, no runtime overhead
            - Cons: Steeper learning curve, more complex rules to follow, can require more upfront design consideration

    */

    /*
       memory types
       1. Stack Memory
           - Fast access
           - Fixed size
           - LIFO structure
           - Used for primitive types and references
           - lifetime of the vaiables automatically cleaned up when out of scope

       2. Heap Memory
           - Slower access
           - Dynamic size
           - No specific order
           - Used for complex data structures and objects
           - we control the lifetime of data on the heap

    */

    /* ------------------------------------------------------------------ */
    // rust ownership rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    /* ------------------------------------------------------------------ */

    // example of scope
    {
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
        println!("{}", s);
    } // this scope is now over, and s is no longer valid
}
