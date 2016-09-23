(function() {
  const whitespace = [' ', '\t'];
  const endOfSource = Symbol();

  function parse(source) {
    const ast = [];
    let instruction;
    while(instruction = parseInstruction(source)) {
      ast.push(instruction);
    }
    return ast;
  }

  function parseInstruction(source) {
    eatWhitespace(source);
    const position = source.position();
    let label = parseSymbol(source);
    if (source.peek() !== ':') {
      source = new ConcatSource(new StringSource(label.value, position), source);
      label = null;
    } else {
      // eat ':'
      source.pop();
    }
    eatWhitespace(source);
    switch(source.peek()) {
      case endOfSource:
        return;
      case ';':
        eatComment(source);
        return parseInstruction(source);
      case '\n':
        source.pop();
        return parseInstruction(source);
      case '.':
        const instr = parseAssemblerInstruction(source);
        instr.label = label;
        return instr;
      case null:
        return eof;
      default:
        return {
          type: 'cpuInstruction',
          label,
          operandA: parseExpression(source),
          operandB: parseExpression(source),
          target: parseExpression(source),
          jump: parseExpression(source),
          position
        };
    }
  }

  function parseAssemblerInstruction(source) {
    eatWhitespace(source);
    const position = source.position();
    const instruction = parseString(source);
    return {
      type: 'asmInstruction',
      instruction,
      addr: parseExpression(source),
      position
    };
  }

  function parseExpression(source) {
    eatWhitespace(source);
    const position = source.position();
    const op1 = parseExpression2(source);
    if (['+', '-'].indexOf(source.peek()) === -1) {
      return op1;
    }
    const op = source.pop();
    const op2 = parseExpression(source);
    return {
      type: 'op',
      op,
      ops: [op1, op2],
      position
    }
  }

  function parseExpression2(source) {
    eatWhitespace(source);
    const position = source.position();
    const op1 = parseExpression3(source);
    if (['*', '/'].indexOf(source.peek()) === -1) {
      return op1;
    }
    const op = source.pop();
    const op2 = parseExpression2(source);
    return {
      type: 'op',
      op,
      ops: [op1, op2],
      position
    }
  }

  function parseExpression3(source) {
    const position = source.position();
    switch(source.peek()) {
      case endOfSource:
        throw new Error(`Unexpcted end of expression at ${source.position()}`);
      case '(':
        source.pop();
        const op = parseExpression(source);
        if(source.pop() !== ')') {
          throw new Error(`Missing parenthesis at ${source.position()}`);
        }
        return op;
      case '-':
        source.pop();
        return {
          type: 'op',
          op: '-',
          ops: parseExpression2(source),
          position
        };
      case '$':
        source.pop();
        return {
          type: 'symbol',
          value: '$',
          position
        };
      case '"':
        return {
          type: 'string',
          value: parseString(source),
          position
        };
      default:
        if('1234567890'.indexOf(source.peek()) !== -1) {
          return {
            type: 'numberLiteral',
            value: parseNumber(source),
            position
          };
        }
        if("acbdefghijklmnopqrstuvwxyz".indexOf(source.peek()) !== -1) {
          return {
            type: 'symbol',
            value: parseSymbol(source),
            position
          };
        }
        throw new Error(`Unexpected character '${source.peek()}' at ${source.position()}`);
    }
  }

  function parseSymbol(source) {
    eatWhitespace(source);
    const position = source.position();
    let value = '';
    while(source.peek() !== endOfSource && /[a-zA-Z0-9_-]/.test(source.peek())) {
      value += source.pop();
    }
    return {
      type: 'symbol',
      value,
      position
    };
  }

  function parseNumber(source) {
    eatWhitespace(source);
    let value = '';
    while (/[0-9a-fA-Fx]/.test(source.peek())) {
      value = source.pop();
    }
    return value;
  }

  function parseString(source) {
    eatWhitespace(source);
    const position = source.position();
    let value = '';
    if(s.pop() !== '"') {
      throw new Error(`String literal didn't start with " at ${source.position()}`);
    }
    let c;
    // Collect chars in `lit` until we reach the other quote
    while(c = s.pop() !== '"') {
      // If we find a blackslash, take the character after the
      // backslash verbatim.
      if(c === '\\') {
        c = s.pop();
      }
      value += c;
    }
    // Eat closing quite
    s.pop();
    return {
      type: 'stringLiteral',
      value,
      position
    };
  }

  function eatComment(source) {
    eatWhitespace(source);
    if(source.peek() !== ';') {
      throw new Error(`Comment didn’t start with ; at ${source.position()}`);
    }
    while(source.peek() !== '\n') {
      source.pop();
    }
    source.pop();
  }

  function eatWhitespace(source) {
    while(whitespace.indexOf(source.peek()) !== -1) {
      source.pop();
    }
  }

  class Position {
    constructor(filename = '', line = 0, character = 0) {
      this.filename = filename;
      this.line = line;
      this.character = character;
    }

    toString() {
      return JSON.stringify({filename: this.filename, line: this.line, character: this.character});
    }
  }

  class StringSource {
    constructor(str, initPosition = new Position()) {
      this.position_ = initPosition;
      this.data_ = Array.from(str);
    }

    peek() {
      if(!this.data_) {
        throw new Error("Unexpected EOF");
      }
      if(this.data_.length === 0) {
        return endOfSource;
      }
      return this.data_[0];
    }

    pop() {
      if(!this.data_) {
        throw new Error("Unexpected EOF");
      }
      const c = this.data_.shift();
      if (c === undefined) {
        this.data_ = null;
        return endOfSource;
      }
      if (c === '\n') {
        this.position_.line++;
        this.position_.character = 0;
        return c;
      }
      this.position_.character++;
      return c;
    }

    position() {
      return this.position_;
    }
  }

  class ConcatSource {
    constructor(...sources) {
      this.sources_ = sources;
      this.purgeEmptySources_();
    }

    peek() {
      if(!this.sources_) {
        throw new Error("Unexpected EOF");
      }
      if(this.sources_.length === 0) {
        return endOfSource;
      }
      return this.sources_[0].peek();
    }

    pop() {
      if(!this.sources_) {
        throw new Error("Unexpected EOF");
      }
      if(this.sources_.length === 0) {
        this.sources_ = null;
        return endOfSource;
      }

      const c = this.sources_[0].pop();
      this.purgeEmptySources_();
      return c;
    }

    position() {
      if(this.sources_.length === 0) {
        return new Position();
      }

      return this.sources_[0].position();
    }

    purgeEmptySources_() {
      while(this.sources_.length > 0 && this.sources_[0].peek() === endOfSource) {
        this.sources_.shift();
      }
    }
  }

  function stripPositions(obj) {
    if(!obj || typeof obj !== 'object') {
      return;
    }
    if('position' in obj) {
      delete obj.position;
    }
    for(let key in obj) {
      stripPositions(obj[key]);
    }
  }

  module.exports = {
    parse,
    StringSource,
    ConcatSource,
    endOfSource,
    stripPositions
  };
})();