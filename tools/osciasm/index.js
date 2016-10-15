const osciasm = require('./osciasm');

const program = require('commander');
const fs = require('mz/fs');

program
  .arguments('<file>')
  .action(file => {
    fs.readFile(file)
      .then(contents => contents.toString('utf-8'))
      .then(contents => osciasm.parse(new osciasm.StringSource(contents)))
      .then(instructions => osciasm.assemble(instructions))
      .then(binary => console.log(binary))
      .catch(error => {
        console.error(`Error while parsing: ${error.toString()}\n${error.stack}`);
      });
  })
  .parse(process.argv);
