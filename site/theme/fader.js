const faderSelectors = [
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

const prepLineFades = () => {
  c.sets.forEach((faderSet, faderSetIndex) => {
    faderSet.fadeLines.forEach((fadeLine, fadeLineIndex) => {
      const playgroundNumber = faderSet + 1
      const lineNumber = fadeLineIndex + 1

      const fadeString = `.ace_line:nth-child(${lineNumber}) span { border: 1px solid red; }`
      c.faderStyles.push(fadeString)

      // const fadeElements = document.querySelectorAll(
      //   '#content > main > pre:nth-child(6) > pre > code > div.ace_scroller > div > div.ace_layer.ace_text-layer > div > span'
      // )
      // fadeElements.forEach((fadeElement, fadeElementIndex) => {
      //   fadeElement.style.border = '1px solid red'
      // })
    })

    // console.log(lineSet)
    // console.log(c.playgrounds[lineSetIndex])
    // console.log(c.playgrounds[0].querySelector('.ace_line:nth-child(1)'))
    // c.playgrounds[0]
    //   .querySelector('.ace_line')
    //   .classList.remove('ace-tomorrow-night')
    // c.playgrounds[0].querySelector('.ace_line').classList.remove('ace_keyword')
  })
}

const getPlaygrounds = () => {
  c.playgrounds = document.querySelectorAll('pre.playground')
}

const makeStyleSheet = () => {
  c.styleOverride = document.createElement('style')
  document.body.appendChild(c.styleOverride)
}

const updateStyles = () => {
  c.styleOverride.innerHTML = c.faderStyles.join('\n')
}

const faderInit = () => {
  if (c) {
    console.log('Got fader config')
    c.faderStyles = []
    makeStyleSheet()
    getPlaygrounds()
    prepLineFades()
    updateStyles()
    console.log(c)
  }
}

document.addEventListener('DOMContentLoaded', faderInit)
