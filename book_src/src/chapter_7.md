# Breaking Spaces

Rdxl macros try to guess when inserting breaking spaces
would be appropriate. This is quite difficult and 
absurd when it comes to statements. For that reason,
most statements just emit breaking spaces regardless
of whether there are any spaces in the input.

The implementation of breaking space detection is
reliant on the [span location](https://doc.rust-lang.org/proc_macro/struct.Span.html)
information provided to procedural macros. This
feature has been under ongoing development, so
there may be quirks from Rust version to version.
