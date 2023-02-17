---> TITLE

Introduction

---> CONTENT

One of the ways I learn is by doing write-ups of
whatever I'm working on. Right now, that's rust.
I'm putting this site together to collect them.
You may find them helpful if you learn the way
I do.

Most pages have examples on them where I show
a full set of source code then show how to
put it together step-by-step. Here's an example
with "Hello World"

---> SOURCE

fn main() {
  println!("Hello, World");
}

---> OUTPUT

lines: 1, 2, 3
highlights: 1, 2, 3


<!--


---

Full source example

~~

lines: 1, 2, 3
highlights: 1, 2, 3

---

Create the `main` function

~~

lines: 1, 3,
highlights: 1, 2, 3

~~~

All the examples on this site are
full programs. Each one starts with
a `main` function which is where
Rust starts to access the code
when it starts the program.


---

Add `println!()`

~~~

lines: 1, 2, 3
highlights: 2

~~~

The `println!()` expression is what Rust uses
to output to the terminal.

-->
