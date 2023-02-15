# Introduction

I'm in the process of learning Rust. I
learn best by writing things up so I'm putting this site
together as my notes. The style is shorter than
[The Rust Book](https://doc.rust-lang.org/book/) but longer
than [Rust By Example](https://doc.rust-lang.org/rust-by-example/).
The general format is to provide a brief explanation then a code
sample followed by more details. The code samples start
and end with a snapshot of the full code sample with
the step by step process described in between.

Here's a basic "Hello, World" program as an example.

<pre id="thing" class="playground"></pre>

---

<pre class="playground">
<div class="buttons">
<button class="fa fa-play play-button" hidden="" title="Run this code" aria-label="Run this code">
</button>
</div>
<code class="language-rust hljs">fn main { println!() }</code>
</pre>

```rust, editable
fn main() {
```

<article id="wrapper" />

<script>

const m = () => {

makeElement(
'pre',
'pre1', 
`<div class="buttons">
<button class="fa fa-play play-button" hidden="" title="Run this code" aria-label="Run this code">
</button>
</div>
<code class="language-rust hljs">HERERERE</code>
`,
'content',
null, null, 'playground')

}

document.addEventListener("DOMContentLoaded", m)


const x = {
  source: `fn main() {
  println!("Hello, World");
}`,
  sets: [
    {
      header: `Full Example`,
      lines: [1, 2, 3],
      highlights: [1, 2, 3],
      rowHighlights: [1, 2, 3],
      overrides: [],
      fades: [],
      note: `This is the full source code for \`Hello, World\``,
    },

    {
      header: `Create the <code>main</code> function`,
      lines: [1, 3],
      highlights: [1, 3],
      rowHighlights: [1, 2, 3],
      overrides: [],
      fades: [],
      note: `Every example on this site is a full Rust program. They all 
start with a \`main\` function which is where Rust kicks them off when 
it starts a program`,
    },

    {
      header: `Stub a \`println!()\` expression`,
      lines: [1, 2, 3],
      highlights: [2],
      rowHighlights: [1, 2, 3],
      overrides: [{ line: 2, text: `  println!();` }],
      fades: [] ,
      note: `The \`println!()\` expression prints a line out to the terminal`,
    },

    {
      header: `Populate \`println!()\``,
      lines: [1, 2, 3],
      highlights: [2],
      rowHighlights: [1,2,3],
      overrides: [],
      fades: [{ line: 2, spans: [1, 2, 3, 4,] }],
      note: `The contents inside the quotes is what \`println!()\` outputs when it's called`,
    },

    {
      header: `Playground`,
      lines: [1, 2, 3],
      highlights: [1,2, 3],
      rowHighlights: [1,2, 3],
      overrides: [],
      fades: [],
      note: ``,
    },


],
}
</script>
