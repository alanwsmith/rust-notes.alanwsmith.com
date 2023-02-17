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
  '.ace_name',
  '.ace_entity.ace_name.ace_function',
  '.ace_line',
]

const prepLineFades = () => {
  c.sets.forEach((faderSet, faderSetIndex) => {
    faderSet.fadeLines.forEach((fadeLine) => {
      const playgroundNumber = faderSet + 1
      const lineNumber = fadeLine
      faderSelectors.forEach((faderSelector) => {
        const fadeString = `#playground${faderSetIndex} .ace_line:nth-child(${lineNumber}) ${faderSelector} { color: var(--fader-font-color); }`
        c.faderStyles.push(fadeString)
      })
      c.faderStyles.push(
        `#playground${faderSetIndex} .ace_line:nth-child(${lineNumber}) { color: var(--fader-font-color); }`
      )
    })
  })
}

const prepWordFades = () => {
  c.sets.forEach((faderSet, faderSetIndex) => {
    faderSet.fadeWordSets.forEach((fadeWordSet) => {
      fadeWordSet.words.forEach((fadeWord) => {
        c.faderStyles.push(
          `#playground${faderSetIndex} .ace_line:nth-child(${fadeWordSet.line}) span:nth-child(${fadeWord}) { color: var(--fader-font-color); }`
        )
        c.faderStyles.push(
          `#playground${faderSetIndex} .ace_line:nth-child(${fadeWordSet.line}):not(span) { color: var(--fader-font-color); }`
        )
        console.log(fadeWordSet.line)
        console.log(fadeWord)
      })
    })
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

const addClasses = () => {
  document
    .querySelectorAll('main pre.playground')
    .forEach((item, itemIndex) => {
      item.id = `playground${itemIndex}`
    })
}

const faderInit = () => {
  if (c) {
    console.log('Got fader config')
    c.faderStyles = []
    makeStyleSheet()
    addClasses()
    getPlaygrounds()
    prepLineFades()
    prepWordFades()
    updateStyles()
    console.log(c)
  }
}

document.addEventListener('DOMContentLoaded', faderInit)
