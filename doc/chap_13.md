## function language features in rust: iterators and closures
* closures: a function-like construct you can store in a variable
* iterators: a way of processing a series of elements

### closures
Rust's closures are anonymous functions that you can save in a variable or pass as arguments to other functions.

To define a closure, we start with a pair of vertical pipes(|). Inside the pipe is where we specify the parameters to the closure;
After the parameter, we put curly braces that hold the body of the closure. The curly braces are optional if the closure body has one line.
The reason we're using a closure is because we want to define the code to call at one point, store that code, and actually call it at a later point.
Closures diff form functions defined with the *fn* keyword in a few ways. The first is that closures don't require you to annotate the types of the parameters or the return value like *fn* function do.
Type annotations are required on functions because they are part of an explicit interface exposed to your users. Defining this interface rigidly is important for ensuring that everyone agrees on what types
of values a function uses and returns. Closures aren't used in an exposed interface like this, though: they're stored in variables and used without naming them an exposing them to be invoked by users of our library.
Additionally, closures are usually short and only relevant within a narrow context rather than in any arbitrary scenario.
Within these limited contexts, the compiler is reliably able to infer the types of the parameters and return type similarly to how it's able to infer the types of most variable.
Being forced to annotate the types in these small, anonymous functions would be annoying and largely redundant with the information the compiler already has avaliable.
The syntax of closures and functions looks more similar with type annotations.
Closure definitions will have one concrete type inferred for each of their parameters and for their return value.
