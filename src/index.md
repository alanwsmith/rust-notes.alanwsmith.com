# Welcome To Rust Notes

Folks learn in different ways. For me, the best way
is writing and publishing notes. 
The process of explaining things cements 
them in my head. 

I'm currently learning Rust. I've decided
to amp things up a bit and build an entire site 
for to the notes. While it's not exactly 
a tutorial, I'm putting the site in an
order designed to work from start to finish. 

I hope you find it useful, 

-alan


## How It Works

Most pages on the site have examples. They start
with the full source code followed by a step
by step guide and notes. The last block of 
code shows the full source again. That one
is editable and you can run it directly on 
the site. Here's "Hello, World" to get you 
started. Feel free to experiment.

<div class="full_source">

### Full Source Code

```rust, editable
fn main() {
  println!("Hello, World");
}
```

<div class="note"></div>

</div>

<h2>Step By Step</h2><div class="example">

### Add the `main` function

```rust, editable
fn main() {

}
```

<div class="note">The examples on this site are full programs. 
They all begin with a `main` function which
is what Rust uses to start things off by
default.</div>

</div>

<div class="example">

### Stub a print line expression

```rust, editable
fn main() {
  println!("Hello, World");
}
```

<div class="note">Printing to the terminal is done with 
the `println!()` expression. The `!`
in there means `println` is a type
of expression called a `macro`. We need
to cover a few other topics before those
will make sense. For now, just know
that the `!` is required and it's 
easy to miss.</div>

</div>

<div class="example">

### Add the string to print

```rust, editable
fn main() {
  println!("Hello, World");
}
```

<div class="note">The content inside the quotes of a 
`println!()` expression is called a 
format string. This one just has
the `Hello, World` text in it. It's 
also possible to output variables
which we'll see shortly.</div>

</div>

<div class="example">

### Run it

```rust, editable
fn main() {
  println!("Hello, World");
}
```

<div class="note">This is the full example again. 
It's editable and you can run
it directly on the site. Play
around with it and don't worry 
about breaking anything. It's 
easy to cause giant error messages
but you won't hurt anything.</div>

</div>

 <script>const c = { sets: [ { fadeWordSets: [],fadeLines: []},{ fadeWordSets: [],fadeLines: []},{ fadeWordSets: [],fadeLines: [1,3,]},{ fadeWordSets: [],fadeLines: [1,3,]},{ fadeWordSets: [],fadeLines: []},] }; </script>