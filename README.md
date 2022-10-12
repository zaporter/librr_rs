# Librr-rs

This project is devoted to building a programmatic Rust interface for the [rr debugging framework](https://github.com/rr-debugger/rr). This project has two major pain points:
- rr is built as an executable rather than as a library. This has lead to many design decisions that make it very painful to add a programmatic interface ontop of. If I am confident enough in my interface, I may eventually consider a PR to rr that stabilizes a C++ interface.
- Rust and C++ interop is still in its infancy and this leads to very unexpected (and painful) behavior depending on where code is written and how things are built.


# Current Status

At this point in time (and for many months to come), I **strongly reccomend against** using this library as it is subject to radical API changes while I figure things out. If you have a use case you want me to consider, please message me or submit a PR outlining your use case. 


# Known Future API Changes
- Some functions return T when they should return Result\<T,E\>
- Cxx currently does not support the Option\<T\> return type. Once it does, some functions should return an option instead of an error code 
- Some functions crash due to C++ assert!() statements when they should return errors. 
- This whole thing needs to be refactored into a main package and a *-sys package to adhere to rust naming standards
