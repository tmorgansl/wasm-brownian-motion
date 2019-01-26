const express = require('express')
const path = require('path')
const PORT = process.env.PORT || 8080

express.static.mime.types['wasm'] = 'application/wasm'

express()
  .use(express.static(path.join(__dirname, '../dist')))
  .get('/', (req, res) => res.render('index.html'))
.listen(PORT, () => console.log(`Listening on ${ PORT } `))