# Introduction

One of the ways I learn is by doing write-ups of
whatever I'm working on. Right now, that's rust.
I'm putting this site together to collect them.
You may find them helpful if you learn the way
I do.

Most pages have examples on them where I show
a full set of source code then show how to
put it together step-by-step. Here's an example
with "Hello World"

### Full Source Code
```rust, editable
fn main() {
  println!("Hello, World");
}
```

This is the full source code for
`Hello, World`. You can run it or 
mess with it here or in last example
below. 

Here's the step-by-step
build process.

### Add the `main` function
```rust, editable
fn main() {

}
```

The samples on the site are full, runnable
programs. They start with this `main` function
which is where Rust kicks things off in 
a program.

### Stub out a print line expression
```rust, editable
fn main() {
  println!();
}
```

Printing a line to the terminal is done
with `println!()` expressions. The `!` means
`println` is a "macro". We need to cover a few
more things before that makes much sense. For now, 
just know that the `!` needs to be there and it's 
super easy to miss.

### Fill in the text
```rust, editable
fn main() {
  println!("Hello, World");
}
```

The area in quotes in a `println!()` 
expression is called a "format string".
We'll look at how to add variables 
to is shortly.

### Run it
```rust, editable
fn main() {
  println!("Hello, World");
}
```

Here's another copy of the code you can play
around with and run directly on the site.

 <script>const c = { sets: [ { fadeWordSets: [],fadeLines: []},{ fadeWordSets: [],fadeLines: []},{ fadeWordSets: [],fadeLines: [1,3,]},{ fadeWordSets: [{line: 2,words: [1, ]},],fadeLines: [1,3,]},{ fadeWordSets: [],fadeLines: []},] }; </script>