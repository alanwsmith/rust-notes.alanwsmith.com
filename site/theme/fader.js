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
        const fadeString = `#playground${faderSetIndex} .ace_line:nth-child(${lineNumber}) ${faderSelector} { color: #333; }`
        c.faderStyles.push(fadeString)
      })
      c.faderStyles.push(
        `#playground${faderSetIndex} .ace_line:nth-child(${lineNumber}) { color: #333; }`
      )
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
    updateStyles()
    console.log(c)
  }
}

document.addEventListener('DOMContentLoaded', faderInit)
