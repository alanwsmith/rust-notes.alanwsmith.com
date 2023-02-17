const prepFadeWords = () => {
  const selectors = [
    '.ace_keyword',
    '.ace_lparen',
    '.ace_source',
    '.ace_rust',
    '.ace_paren',
    '.ace_function',
    '.ace_entity',
    '.ace_identifier',
    '.ace_operator',
    '.ace_support',
    '.ace_constant',
    '.ace_quoted',
    '.ace_string',
    '.ace_double',
    '.ace_punctuation',
    '.ace_rparen',
  ]

  c.sets.forEach((set, setIndex) => {
    set.fades.forEach((fade, fadeIndex) => {
      fade.spans.forEach((span, spanIndex) => {
        selectors.forEach((selector) => {
          const line = `#sectionCode${setIndex}.ace-monokai .ace_line:nth-child(${fade.line}) span:nth-child(${fade.spans[spanIndex]})${selector} { color: var(--fade-color); }`
          c.highlightRemovers.push(line)
        })
      })
    })
  })
}

const makeElement = (
  _type,
  _id,
  _html,
  _childOf,
  _event,
  _function,
  _classes
) => {
  const newElement = document.createElement(_type)
  newElement.id = _id
  newElement.innerHTML = _html
  window[_childOf].appendChild(newElement)
  if (_event !== null) {
    newElement.addEventListener(_event, _function)
  }
  if (_classes) {
    newElement.classList.add(_classes)
  }
  return newElement
}

const makeSections = () => {
  c.sets.forEach((set, setIndex) => {
    const section = makeElement(
      'section',
      `section${setIndex}`,
      '',
      'wrapper',
      null,
      null,
      'codeSection'
    )

    const header = makeElement(
      'h3',
      `sectionHeader${setIndex}`,
      set.header,
      `section${setIndex}`,
      null,
      null,
      'codeSectionHeader'
    )

    const code = makeElement(
      'pre',
      `sectionCode${setIndex}`,
      // ``,

      `
<div class="buttons"><button class="fa" hidden="" title="Run" aria-label="Run">Run</button></div>

      <code id="codeBlock${setIndex}" class="editable ace_editor ace_hidpi hljs ace-tomorrow-night ace_dark">

      ${setIndex}" class="language-rust">${c.sets[setIndex].outputLines.join(
        '\n'
      )},

      </code>`,

      // `<pre class="playground"><code id="codeBlock${setIndex}" class="language-rust">${c.sets[
      //   setIndex
      // ].outputLines.join('\n')}</code></pre>`,

      `section${setIndex}`,

      null,
      null,
      'playground'
    )

    // const note = makeElement(
    //   `div`,
    //   `sectionNote${setIndex}`,
    //   set.note,
    //   `section${setIndex}`,
    //   null,
    //   null,
    //   'codeSectionNote'
    // )

    set.editor = ace.edit(`codeBlock${setIndex}`)
    set.editor.setOption('maxLines', 1000)

    // set.editor = ace.edit(`sectionCode${setIndex}`)
    // set.editor.setOption('maxLines', 1000)
    // set.editor.setTheme('ace/theme/monokai')
    // set.editor.session.setMode('ace/mode/rust')
    // set.editor.setHighlightActiveLine(false)
    // set.editor.setOptions({
    //   readOnly: true,
    //   highlightActiveLine: false,
    //   highlightGutterLine: false,
    // })

    // set.editor.renderer.$cursorLayer.element.style.display = 'none'
  })
}

const prepLines = () => {
  c.sets.forEach((set, setIndex) => {
    set.outputLines = []
    c.rawLines.forEach((line, lineIndex) => {
      if (set.lines.includes(lineIndex + 1)) {
        set.outputLines.push(line)
      } else {
        set.outputLines.push(' ')
      }
    })
  })
}

const prepOverrides = () => {
  c.sets.forEach((set, setIndex) => {
    if (set.overrides) {
      set.overrides.forEach((override) => {
        set.outputLines[override.line - 1] = override.text
      })
    }
  })
}

const prepFadeLines = () => {
  const selectors = [
    '.ace_keyword',
    '.ace_lparen',
    '.ace_source',
    '.ace_rust',
    '.ace_paren',
    '.ace_function',
    '.ace_entity',
    '.ace_identifier',
    '.ace_operator',
    '.ace_support',
    '.ace_constant',
    '.ace_quoted',
    '.ace_string',
    '.ace_double',
    '.ace_punctuation',
    '.ace_rparen',
  ]

  c.sets.forEach((set, setIndex) => {
    c.rawLines.forEach((line, lineIndex) => {
      if (!set.highlights.includes(lineIndex + 1)) {
        for (let selector of selectors) {
          c.highlightRemovers.push(
            `#sectionCode${setIndex} .ace_line:nth-child(${
              lineIndex + 1
            }) ${selector} { color: var(--fade-color) }`
          )
        }
      }
    })
  })
}

const prepRowHighlights = () => {
  c.sets.forEach((set, setIndex) => {
    set.rowHighlights.forEach((rowHighlight, rowHighlightIndex) => {
      const style = `#sectionCode${setIndex} .ace_line:nth-child(${rowHighlight}) { background-color: var(--row-highlight);}`
      // TODO: rename this to `highlightUpdaters`
      c.highlightRemovers.push(style)
    })
  })
}

const init = () => {
  if (typeof c !== 'undefined') {
    c.rawLines = c.source.split('\n')
    c.highlightRemovers = []
    prepLines()
    prepOverrides()
    makeSections()
    c.styleOverride = document.createElement('style')
    document.body.appendChild(c.styleOverride)
    prepFadeLines()
    prepFadeWords()
    prepRowHighlights()
    c.styleOverride.innerHTML = c.highlightRemovers.join('\n')
  }
}

document.addEventListener('DOMContentLoaded', init)
