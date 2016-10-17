(function() {
  const whitespace = [' ', '\t'];
  const endOfSource = Symbol();

  function parse(source) {
    const ast = [];
    let instruction;
    while(instruction = parseInstruction(source)) {
      switch(instruction.type) {
        case 'cpuInstruction':
          ['operandA', 'operandB', 'target', 'jump'].forEach(field =>
            instruction[field] = buildRPN(tokenizeExpression(instruction[field]))
          );
        break;
        case 'asmInstruction':
            instruction.ops = instruction.ops.map(expr => buildRPN(tokenizeExpression(expr)));
        break;
      }
      ast.push(instruction);
    }
    return ast;
  }

  function parseInstruction(source) {
    eatWhitespace(source);
    const position = source.position();
    const label = parseSymbol(source);
    if (source.peek() !== ':') {
      source = new ConcatSource(new StringSource(label.value, position), source);
    } else {
      // eat ':'
      source.pop();
      return {
        type: 'label',
        value: label.value
      };
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
        return parseAssemblerInstruction(source);
      case null:
        return eof;
      default:
        return {
          type: 'cpuInstruction',
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
    if(source.pop() !== '.') {
      throw new Error(`Assembler instruction didn't start with . at ${position}`);
    }
    const instruction = parseSymbol(source).value;
    const ops = [];
    eatWhitespace(source);
    while(source.peek() !== endOfSource && ['\n', ';'].indexOf(source.peek()) === -1) {
      ops.push(parseExpression(source));
    }
    return {
      type: 'asmInstruction',
      instruction,
      ops,
      position
    };
  }

  function parseExpression(source) {
    eatWhitespace(source);
    const position = source.position();
    const op1 = parseExpression2(source);
    const op2 = parseExpressionPrime(source);
    if(!op2) {
      return op1;
    }
    return {
      type: 'op',
      op: 'expr',
      ops: [op1, op2],
      position
    };
  }

  function parseExpressionPrime(source) {
    eatWhitespace(source);
    const position = source.position();
    if (source.peek() === endOfSource || ['+', '-'].indexOf(source.peek()) === -1) {
      return null;
    }
    const op = source.pop();
    const op2 = parseExpression2(source);
    const op3 = parseExpressionPrime(source);
    return {
      type: 'op',
      op,
      ops: [op2, op3].filter(x => !!x),
      position
    }
  }

  function parseExpression2(source) {
    eatWhitespace(source);
    const position = source.position();
    const op1 = parseExpression3(source);
    const op2 = parseExpression2Prime(source);
    if(!op2) {
      return op1;
    }
    return {
      type: 'op',
      op: 'expr',
      ops: [op1, op2],
      position
    }
  }

  function parseExpression2Prime(source) {
    eatWhitespace(source);
    const position = source.position();
    if (source.peek() === endOfSource || ['*', '/'].indexOf(source.peek()) === -1) {
      return null;
    }
    const op = source.pop();
    const op2 = parseExpression3(source);
    const op3 = parseExpression2Prime(source) ;
    return {
      type: 'op',
      op,
      ops: [op2, op3].filter(x => !!x),
      position
    }
  }

  function parseExpression3(source) {
    eatWhitespace(source);
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
        return parseString(source);
      default:
        if('1234567890'.indexOf(source.peek()) !== -1) {
          return {
            type: 'numberLiteral',
            value: parseNumber(source),
            position
          };
        }
        if("acbdefghijklmnopqrstuvwxyz".indexOf(source.peek()) !== -1) {
          return parseSymbol(source);
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
    while (source.peek() !== endOfSource && /[0-9a-fA-Fx]/.test(source.peek())) {
      value += source.pop();
    }
    return value;
  }

  function parseString(source) {
    eatWhitespace(source);
    const position = source.position();
    let value = '';
    if(source.pop() !== '"') {
      throw new Error(`String literal didn't start with " at ${source.position()}`);
    }
    let c;
    // Collect chars in `lit` until we reach the other quote
    while((c = source.pop()) !== '"') {
      // If we find a blackslash, take the character after the
      // backslash verbatim.
      if(c === '\\') {
        c = source.pop();
      }
      value += c;
    }
    // Eat closing quite
    source.pop();
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

  function tokenizeExpression(expr) {
    switch(expr.type) {
      case 'op':
        switch(expr.op) {
          case '+':
          case '-':
          case '*':
          case '/':
            return expr.ops.map(tokenizeExpression).reduce((prev, cur) => [...prev, ...cur], [{'type': 'op', op: expr.op}]);
          case 'expr':
            return ['(', ...tokenizeExpression(expr.ops[0]), ...tokenizeExpression(expr.ops[1]), ')'];
        }
      case 'symbol':
      case 'numberLiteral':
      case 'stringLiteral':
        return [{'type': expr.type, value: expr.value}];
      default:
        throw new Error(`Invalid token ${expr} in expression at ${expr.position}`);
    }
  }

  // Shunting-yard algorithm
  function buildRPN(tokens) {
    const priority = {
      '+': 0,
      '-': 0,
      '*': 1,
      '/': 1
    };
    const operators = Object.keys(priority);
    const opstack = [];
    // This way opstack[0] ~= opstack.peek()
    opstack.push = opstack.unshift;
    opstack.pop = opstack.shift;
    const output = [];

    for(let token of tokens) {
      if(token === '(') {
        opstack.push(token);
      } else if(token === ')') {
        while(opstack.length > 0 && opstack[0] !== '(') {
          output.push(opstack.pop());
        }
        opstack.pop();
      } else if(operators.indexOf(token.op) !== -1) {
        while(opstack.length > 0 && priority[token.op] <= priority[opstack[0].op]) {
          output.push(opstack.pop());
        }
        opstack.push(token);
      } else {
        output.push(token);
      }
    }
    return output;
  }

  /*
   *
   * FIXME: Pull in data from emulator.h
   */
  function defaultStartState() {
    return {
      symbols: {
        '$': 0,
        'instruction_size': 4*4,
        'register3': 0xFFFFFFFF - 1*4,
        'register2': 0xFFFFFFFF - 2*4,
        'register1': 0xFFFFFFFF - 3*4,
        'register0': 0xFFFFFFFF - 4*4,
        'ivt0': 0xFFFFFFFF - 4*4 - 1*4,
        'flags0': 0xFFFFFFFF - 4*4 - 1*4 - 1*4
      }
    }
  }

  function evaluateRPN(rpn, symbolTable) {
    const token = rpn.pop();
    switch(token.type) {
      case 'numberLiteral':
        if (token.value.startsWith('0x')) {
          return parseInt(token.value.substr(2), 16);
        }
        else if (token.value.startsWith('0b')) {
          return parseInt(token.value.substr(2), 2);
        }
        else if (token.value.startsWith('0')) {
          return parseInt(token.value, 8);
        }
        return parseInt(token.value, 10);
      case 'symbol':
        if(token.value in symbolTable)
          return symbolTable[token.value];
        throw new Error(`Unknown symbol ${token.value}`);
      case 'op':
        const op1 = evaluateRPN(rpn, symbolTable);
        const op2 = evaluateRPN(rpn, symbolTable);
        switch(token.op) {
          case '+':
            return op1+op2;
          case '-':
            return op1-op2;
          case '*':
            return op1*op2;
          case '/':
            return Math.floor(op1/op2);
        }
      default:
        throw new Error(`Unknown instruction type ${token.type}`);
    }
  }

  function intToBytes(i) {
    return [
      (i >> 0) & 0xFF,
      (i >> 8) & 0xFF,
      (i >> 16) & 0xFF,
      (i >> 24) & 0xFF
    ];
  }

  function assemble(instructions, state) {
    if(!state) {
      state = defaultStartState();
    }
    let ip = 0;
    for(let instruction of instructions) {
      if(instruction.type === 'asmInstruction' && instruction.instruction === 'addr') {
        ip = evaluateRPN(instruction.ops[0].slice(), state.symbols);
      } else if(instruction.type === 'label') {
        state.symbols[instruction.value] = ip;
      } else {
        ip += sizeOfInstruction(instruction);
      }
    }
    return instructions
            .reduce(
              (program, instr) => [...program, ...assembleInstruction(instr, state)],
              []
            );
  }

  function sizeOfInstruction(instruction) {
    switch(instruction.type) {
      case 'asmInstruction':
        switch(instruction.instruction) {
          case 'db':
            return instruction.ops.length;
          case 'dw':
            return instruction.ops.length*4;
          case 'addr':
            return 0;
          default:
            throw new Error(`Unknown assembler instruction .${instruction.instruction}`);
        }
      case 'cpuInstruction':
        return 4*4;
      default:
        throw new Error(`Unknown instruction type ${instruction.type}`);
    }
  }

  function assembleInstruction(instruction, state) {
    switch(instruction.type) {
      case 'label':
        // Already handled
        return [];
      case 'asmInstruction':
        switch(instruction.instruction) {
          case 'db':
            return instruction.ops.map(op => evaluateRPN(op, state.symbols) % 256);
          case 'dw':
            return [];
          case 'addr':
            if(instruction.ops.length !== 1) {
              throw new Error(`.addr takes exactly one argument`);
            }
            state.symbols['$'] = evaluateRPN(instruction.ops[0], state.symbols);
            return [];
          default:
            throw new Error(`Unknown assembler instruction .${instruction.instruction}`);
        }
      case 'cpuInstruction':
        const instr = [
          ...intToBytes(evaluateRPN(instruction.operandA, state.symbols)),
          ...intToBytes(evaluateRPN(instruction.operandB, state.symbols)),
          ...intToBytes(evaluateRPN(instruction.target, state.symbols)),
          ...intToBytes(evaluateRPN(instruction.jump, state.symbols))
        ];
        state.symbols['$'] += 4*4;
        return instr;
      default:
        throw new Error(`Unknown instruction type ${instruction.type}`);
    }
  }

  module.exports = {
    parse,
    assemble,
    defaultStartState,
    StringSource,

    // low-level exports
    parseInstruction,
    assembleInstruction,
    ConcatSource,
    endOfSource,
    stripPositions,
    tokenizeExpression,
    buildRPN,
    evaluateRPN
  };
})();