console.log('herex')

const newEl = document.createElement('pre')
newEl.innerHTML = `<pre class="playground">
<code class="editable ace_editor ace_hidpi hljs ace-tomorrow-night ace_dark" id="editor1">fn main() {

println!("yo again");

}
</code></pre>`

window.target1.appendChild(newEl)

editor = ace.edit('editor1')
editor.session.setMode('ace/mode/rust')
editor.setOption('maxLines', 1000)
editor.setValue(`fn main() {
  println!("more");
}`)
