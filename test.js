const { getIncludeFileList, walkdirGetIncludeFileList } = require('./index.js')

// const testPath = '/Volumes/SSD/work'
const testPath = __dirname

let walkdirRes
// for (let i = 0; i < 5; i++) {
  console.time('walkdir')
  walkdirRes = walkdirGetIncludeFileList(testPath)
  console.log('walkdirRes', walkdirRes)
  console.timeEnd('walkdir')
// }

let fsRes
// for (let i = 0; i < 5; i++) {
  console.time('fs')
  fsRes = getIncludeFileList(testPath)
  console.log('fsRes', fsRes)
  console.timeEnd('fs')
// }


console.log('fsRes length', fsRes.length)
console.log('walkdirRes length', walkdirRes.length)