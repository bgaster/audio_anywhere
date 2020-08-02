'use strict'

const fs = require('fs')
const libs = fs.readdirSync('./scripts/lib').filter((file) => { return file.indexOf('.js') > 0 && file !== 'build.js' })
const scripts = fs.readdirSync('./scripts').filter((file) => { return file.indexOf('.js') > 0 })
const styles = fs.readdirSync('./links').filter((file) => { return file.indexOf('.css') > 0 })
const id = process.cwd().split('/').slice(-1)[0]

function cleanup (txt) {
  const lines = txt.split('\n')
  let output = ''
  for (const line of lines) {
    if (line.trim() === '') { continue }
    if (line.trim().substr(0, 2) === '//') { continue }
    if (line.indexOf('/*') > -1 && line.indexOf('*/') > -1) { continue }
    output += line + '\n'
  }
  return output
}

// Create release

fs.writeFileSync('index.html', cleanup(`
<!DOCTYPE html>
<html lang="en">
<html>
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${id}</title>
  </head>
  <body>
    <script>
      ${libs.reduce((acc, item) => { return `${acc}// Including Library ${item}\n\n${fs.readFileSync('./scripts/lib/' + item, 'utf8')}\n` }, '')}
      ${scripts.reduce((acc, item) => { return `${acc}// Including Script ${item}\n\n${fs.readFileSync('./scripts/' + item, 'utf8')}\n` }, '')}
      const client = new Client()
      client.install(document.body)
      window.addEventListener('load', () => { 
        client.start()
        OnParamChange(0, 0.75)
      })

      const paramFilter = 0
      const paramWave = 1
      const paramRelation = 2
      const paramSub = 3

      function OnParamChange(param, value) {
          if (param == paramFilter) {
              client.renderer.updateParamFilter(value)
          }
          else if (param == paramWave) {
              client.renderer.updateParamWave(value)
          }
          else if (param == paramRelation) {
              client.renderer.updateParamRelation(value)
          }
          else if (param == paramSub) {
              client.renderer.updateParamSub(value)
          }
      }

      function OnControlChange(ctrlTag, value) {
      }
    </script>
    <style>
    ${styles.reduce((acc, item) => { return `${acc}/* Including Style ${item} */ \n\n${fs.readFileSync('./links/' + item, 'utf8')}\n` }, '')}
    </style>
  </body>
</html>`))

// Create debug

fs.writeFileSync('debug.html', `
<!DOCTYPE html>
<html lang="en">
<html>
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${id}</title>
    ${styles.reduce((acc, item) => { return `${acc}<link rel="stylesheet" type="text/css" href="./links/${item}"/>\n` }, '')}
    ${libs.reduce((acc, item) => { return `${acc}<script type="text/javascript" src="./scripts/lib/${item}"></script>\n` }, '')}
    ${scripts.reduce((acc, item) => { return `${acc}<script type="text/javascript" src="./scripts/${item}"></script>\n` }, '')}
  </head>
  <body>
    <script>
      const client = new Client()
      client.install(document.body)
      window.addEventListener('load', () => { 
        client.start()
      })
    </script>
  </body>
</html>`)

console.log(`Built ${id}`)
