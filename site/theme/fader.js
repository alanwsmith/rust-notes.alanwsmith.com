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

const fadeLines = () => {
  c.sets.forEach((lineSet, lineSetIndex) => {
    console.log(lineSet)
    console.log(c.playgrounds[lineSetIndex])
    console.log(c.playgrounds[0].querySelector('.ace_line'))
    c.playgrounds[0]
      .querySelector('.ace_line')
      .classList.remove('ace-tomorrow-night')
    c.playgrounds[0].querySelector('.ace_line').classList.remove('ace_keyword')
  })
}

const getPlaygrounds = () => {
  c.playgrounds = document.querySelectorAll('pre.playground')
}

const faderInit = () => {
  if (c) {
    console.log('Got fader config')
    getPlaygrounds()
    fadeLines()
    console.log(c)
  }
}
document.addEventListener('DOMContentLoaded', faderInit)
