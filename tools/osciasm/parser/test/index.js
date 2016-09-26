const chai = require('chai');
const expect = chai.expect;

const parser = require('../');

describe('Parser', function() {
  it('should have a parsing function', function() {
    expect(parser).to.have.property('parse');
  });

  it('should ignore comments', function() {
    const code =
    `
    ; This is a comment
    `;
    expect(parser.parse(new parser.StringSource(code))).to.have.length(0);
  });

  it('should parse instructions', function() {
    const code =
    `
    1 2 3 4
    4 3 2 1
    `;
    const ast = parser.parse(new parser.StringSource(code));
    ast.forEach(i => parser.stripPositions(i))

    expect(ast).to.deep.equal([
      {
        type: 'cpuInstruction',
        operandA: [{
          type: 'numberLiteral',
          value: '1'
        }],
        operandB: [{
          type: 'numberLiteral',
          value: '2'
        }],
        target: [{
          type: 'numberLiteral',
          value: '3'
        }],
        jump: [{
          type: 'numberLiteral',
          value: '4'
        }]
      },
      {
        type: 'cpuInstruction',
        operandA: [{
          type: 'numberLiteral',
          value: '4'
        }],
        operandB: [{
          type: 'numberLiteral',
          value: '3'
        }],
        target: [{
          type: 'numberLiteral',
          value: '2'
        }],
        jump: [{
          type: 'numberLiteral',
          value: '1'
        }]
      }
    ]);
  });

  it('should parse labels before instructions', function() {
    const code =
    `
    label: 1 2 3 4
    `;
    const ast = parser.parse(new parser.StringSource(code));
    ast.forEach(i => parser.stripPositions(i))

    expect(ast).to.deep.equal([
      {
        type: 'label',
        value: 'label'
      },
      {
        type: 'cpuInstruction',
        operandA: [{
          type: 'numberLiteral',
          value: '1'
        }],
        operandB: [{
          type: 'numberLiteral',
          value: '2'
        }],
        target: [{
          type: 'numberLiteral',
          value: '3'
        }],
        jump: [{
          type: 'numberLiteral',
          value: '4'
        }]
      }
    ]);
  });

  it('should parse standalone labels', function() {
    const code =
    `
    label1: ; we use 2 labels
    label2:
      1 2 3 4
    `;
    const ast = parser.parse(new parser.StringSource(code));
    ast.forEach(i => parser.stripPositions(i))

    expect(ast).to.deep.equal([
      {
        type: 'label',
        value: 'label1'
      },
      {
        type: 'label',
        value: 'label2'
      },
      {
        type: 'cpuInstruction',
        operandA: [{
          type: 'numberLiteral',
          value: '1'
        }],
        operandB: [{
          type: 'numberLiteral',
          value: '2'
        }],
        target: [{
          type: 'numberLiteral',
          value: '3'
        }],
        jump: [{
          type: 'numberLiteral',
          value: '4'
        }]
      }
    ]);
  });

  it('should parse assembler instructions', function() {
    const code =
    `
    .addr 256
    .db 127
    `;
    const ast = parser.parse(new parser.StringSource(code));
    ast.forEach(i => parser.stripPositions(i))

    expect(ast).to.deep.equal([
      {
        type: 'asmInstruction',
        instruction: 'addr',
        value: [{
          type: 'numberLiteral',
          value: '256'
        }]
      },
      {
        type: 'asmInstruction',
        instruction: 'db',
        value: [{
          type: 'numberLiteral',
          value: '127'
        }]
      }
    ]);
  });

  it('should handle left-associativity', function() {
    const code =
    `
    .db 1+2+3
    .db 1*2*3
    `;
    const ast = parser.parse(new parser.StringSource(code));
    ast.forEach(i => parser.stripPositions(i))

    expect(ast).to.deep.equal([
      {
        type: 'asmInstruction',
        instruction: 'db',
        value: [
          {type: 'numberLiteral', value: '1'},
          {type: 'numberLiteral', value: '2'},
          {type: 'op', op: '+'},
          {type: 'numberLiteral', value: '3'},
          {type: 'op', op: '+'}
        ]
      },
      {
        type: 'asmInstruction',
        instruction: 'db',
        value: [
          {type: 'numberLiteral', value: '1'},
          {type: 'numberLiteral', value: '2'},
          {type: 'op', op: '*'},
          {type: 'numberLiteral', value: '3'},
          {type: 'op', op: '*'}
        ]
      }
    ]);
  });

  it('should parse "current address" symbol', function() {
    const code =
    `
    .db 2*($+1)
    `;
    const ast = parser.parse(new parser.StringSource(code));
    ast.forEach(i => parser.stripPositions(i))

    expect(ast).to.deep.equal([
      {
        type: 'asmInstruction',
        instruction: 'db',
        value: [
          {type: 'numberLiteral', value: '2'},
          {type: 'symbol', value: '$'},
          {type: 'numberLiteral', value: '1'},
          {type: 'op', op: '+'},
          {type: 'op', op: '*'}
        ]
      }
    ]);
  });

  it('should parse labels', function() {
    const code =
    `
    .db someLabel+2
    `;
    const ast = parser.parse(new parser.StringSource(code));
    ast.forEach(i => parser.stripPositions(i))

    expect(ast).to.deep.equal([
      {
        type: 'asmInstruction',
        instruction: 'db',
        value: [
          {type: 'symbol', value: 'someLabel'},
          {type: 'numberLiteral', value: '2'},
          {type: 'op', op: '+'}
        ]
      }
    ]);
  });

  it('should parse hexadecimal, octal and binary literals', function() {
    const code =
    `
    .db 0xFF - 0777 + 0b111 ; With spaces!
    `;
    const ast = parser.parse(new parser.StringSource(code));
    ast.forEach(i => parser.stripPositions(i))

    expect(ast).to.deep.equal([
      {
        type: 'asmInstruction',
        instruction: 'db',
        value: [
          {type: 'numberLiteral', value: '0xFF'},
          {type: 'numberLiteral', value: '0777'},
          {type: 'op', op: '-'},
          {type: 'numberLiteral', value: '0b111'},
          {type: 'op', op: '+'}
        ]
      }
    ]);
  });

  it('should parse string literals', function() {
    const code =
    `
    .db "Lets try a string"
    `;
    const ast = parser.parse(new parser.StringSource(code));
    ast.forEach(i => parser.stripPositions(i))

    expect(ast).to.deep.equal([
      {
        type: 'asmInstruction',
        instruction: 'db',
        value: [{
          type: 'stringLiteral',
          value: 'Lets try a string'
        }]
      }
    ]);
  });

  it('should parse complex instructions', function() {
    const code =
    `
    loop: someStringLabel+4*someCounter someOtherString+someCounter register0 $+4*12
    `;
    const ast = parser.parse(new parser.StringSource(code));
    ast.forEach(i => parser.stripPositions(i))

    expect(ast).to.deep.equal([
      {
        type: 'label',
        value: 'loop'
      },
      {
        type: 'cpuInstruction',
        operandA: [
          {type: 'symbol', value: 'someStringLabel'},
          {type: 'numberLiteral', value: '4'},
          {type: 'symbol', value: 'someCounter'},
          {type: 'op', op: '*'},
          {type: 'op', op: '+'}
        ],
        operandB: [
          {type: 'symbol', value: 'someOtherString'},
          {type: 'symbol', value: 'someCounter'},
          {type: 'op', op: '+'}
        ],
        target: [
          {type: 'symbol', value: 'register0'}
        ],
        jump: [
          {type: 'symbol', value: '$'},
          {type: 'numberLiteral', value: '4'},
          {type: 'numberLiteral', value: '12'},
          {type: 'op', op: '*'},
          {type: 'op', op: '+'},
        ]
      }
    ]);
  });
});

describe('StringSource', function() {
  it('should return endOfSource on empty string', function() {
    const source = new parser.StringSource('');
    expect(source.peek()).to.equal(parser.endOfSource);
    expect(source.pop()).to.equal(parser.endOfSource);
  });

  it('should throw when consuming beyond endOfSource', function() {
    const source = new parser.StringSource('');
    source.pop();
    expect(source.peek).to.throw();
    expect(source.pop).to.throw();
  });

  it('should yield characters of a string', function() {
    const source = new parser.StringSource('1234');
    expect(source.peek()).to.equal('1');
    expect(source.pop()).to.equal('1');
    expect(source.peek()).to.equal('2');
    expect(source.pop()).to.equal('2');
    expect(source.peek()).to.equal('3');
    expect(source.pop()).to.equal('3');
    expect(source.peek()).to.equal('4');
    expect(source.pop()).to.equal('4');
    expect(source.peek()).to.equal(parser.endOfSource);
    expect(source.pop()).to.equal(parser.endOfSource);
    expect(source.peek).to.throw();
    expect(source.pop).to.throw();
  });

  it('should give the correct position', function() {
    const source =
      new parser.StringSource('1234\n1234', {line: 0, character: 0, filename: 'test.txt'});
    expect(source.position()).to.deep.equal({line: 0, character: 0, filename: 'test.txt'});
    source.pop();
    expect(source.position()).to.deep.equal({line: 0, character: 1, filename: 'test.txt'});
    source.pop();
    expect(source.position()).to.deep.equal({line: 0, character: 2, filename: 'test.txt'});
    source.pop();
    source.pop();
    expect(source.position()).to.deep.equal({line: 0, character: 4, filename: 'test.txt'});
    source.pop();
    expect(source.position()).to.deep.equal({line: 1, character: 0, filename: 'test.txt'});
  });


});

describe('ConcatSource', function() {
  it('should return endOfSource on empty string', function() {
    const source = new parser.ConcatSource();
    expect(source.peek()).to.equal(parser.endOfSource);
    expect(source.pop()).to.equal(parser.endOfSource);
  });

  it('should throw when consuming beyond endOfSource', function() {
    const source = new parser.ConcatSource();
    source.pop();
    expect(source.peek).to.throw();
    expect(source.pop).to.throw();
  });

  it('should yield characters of a string', function() {
    const source = new parser.ConcatSource(
      new parser.StringSource('12'),
      new parser.StringSource('3'),
      new parser.StringSource('4'));
    expect(source.peek()).to.equal('1');
    expect(source.pop()).to.equal('1');
    expect(source.peek()).to.equal('2');
    expect(source.pop()).to.equal('2');
    expect(source.peek()).to.equal('3');
    expect(source.pop()).to.equal('3');
    expect(source.peek()).to.equal('4');
    expect(source.pop()).to.equal('4');
    expect(source.peek()).to.equal(parser.endOfSource);
    expect(source.pop()).to.equal(parser.endOfSource);
    expect(source.peek).to.throw();
    expect(source.pop).to.throw();
  });

  it('should give the correct position', function() {
    const source = new parser.ConcatSource(
      new parser.StringSource('12', {line: 0, character: 0, filename: 'a.txt'}),
      new parser.StringSource('3', {line: 0, character: 0, filename: 'b.txt'}),
      new parser.StringSource('4', {line: 0, character: 0, filename: 'c.txt'})
    );
    expect(source.position()).to.deep.equal({line: 0, character: 0, filename: 'a.txt'});
    source.pop();
    expect(source.position()).to.deep.equal({line: 0, character: 1, filename: 'a.txt'});
    source.pop();
    expect(source.position()).to.deep.equal({line: 0, character: 0, filename: 'b.txt'});
    source.pop();
    expect(source.position()).to.deep.equal({line: 0, character: 0, filename: 'c.txt'});
  });

  it('should handle empty sources at the start', function() {
    const source = new parser.ConcatSource(
      new parser.StringSource(''),
      new parser.StringSource('A')
    );

    expect(source.peek()).to.equal('A');
    expect(source.pop()).to.equal('A');
  });
});

describe('tokenizeExpression', function() {
  it('should tokenize expressions with parenthesis', function() {
    const ast = parser.parseInstruction(new parser.StringSource('.db 2*3*($+1+3)')).value;
    const tokenized = parser.tokenizeExpression(ast);
    expect(tokenized).to.deep.equal([
      '(',
      {type: 'numberLiteral', value: '2'},
      {type: 'op', op: '*'},
      {type: 'numberLiteral', value: '3'},
      {type: 'op', op: '*'},
      '(',
      {type: 'symbol', value: '$'},
      {type: 'op', op: '+'},
      {type: 'numberLiteral', value: '1'},
      {type: 'op', op: '+'},
      {type: 'numberLiteral', value: '3'},
      ')',
      ')'
    ]);
  });

  it('should tokenize expressions with different priorities', function() {
    const ast = parser.parseInstruction(new parser.StringSource('.db 2+3*1')).value;
    const tokenized = parser.tokenizeExpression(ast);
    expect(tokenized).to.deep.equal([
      '(',
      {type: 'numberLiteral', value: '2'},
      {type: 'op', op: '+'},
      '(',
      {type: 'numberLiteral', value: '3'},
      {type: 'op', op: '*'},
      {type: 'numberLiteral', value: '1'},
      ')',
      ')'
    ]);
  });
});

describe('buildRPN', function() {
  it('should convert token arrays to RPN', function() {
    const ast = parser.parseInstruction(new parser.StringSource('.db 2+3*1')).value;
    const tokenized = parser.tokenizeExpression(ast);
    const rpn = parser.buildRPN(tokenized);
    expect(rpn).to.deep.equal([
      {
        type: 'numberLiteral',
        value: '2'
      },
      {
        type: 'numberLiteral',
        value: '3'
      },
      {
        type: 'numberLiteral',
        value: '1'
      },
      {
        type: 'op',
        op: '*'
      },
      {
        type: 'op',
        op: '+'
      }
    ]);
  });
});
