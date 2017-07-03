const chai = require('chai');
const expect = chai.expect;

const osciasm = require('../');

describe('osciasm', function() {
  it('should have a parsing function', function() {
    expect(osciasm).to.have.property('parse');
  });

  describe('parse', function() {
    it('should ignore comments', function() {
      const code =
      `
      ; This is a comment
      `;
      expect(osciasm.parse(new osciasm.StringSource(code))).to.have.length(0);
    });

    it('should parse instructions', function() {
      const code =
      `
      1 2 3 4
      4 3 2 1
      `;
      const ast = osciasm.parse(new osciasm.StringSource(code));
      ast.forEach(i => osciasm.stripPositions(i))

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
      const ast = osciasm.parse(new osciasm.StringSource(code));
      ast.forEach(i => osciasm.stripPositions(i))

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
      const ast = osciasm.parse(new osciasm.StringSource(code));
      ast.forEach(i => osciasm.stripPositions(i))

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
      .db 127 1
      `;
      const ast = osciasm.parse(new osciasm.StringSource(code));
      ast.forEach(i => osciasm.stripPositions(i))

      expect(ast).to.deep.equal([
        {
          type: 'asmInstruction',
          instruction: 'addr',
          ops: [[{
            type: 'numberLiteral',
            value: '256'
          }]]
        },
        {
          type: 'asmInstruction',
          instruction: 'db',
          ops: [
            [{
              type: 'numberLiteral',
              value: '127'
            }],
            [{
              type: 'numberLiteral',
              value: '1'
            }]
          ]
        }
      ]);
    });

    it('should handle left-associativity', function() {
      const code =
      `
      .db 1+2+3 1*2*3
      `;
      const ast = osciasm.parse(new osciasm.StringSource(code));
      ast.forEach(i => osciasm.stripPositions(i))

      expect(ast).to.deep.equal([
        {
          type: 'asmInstruction',
          instruction: 'db',
          ops: [[
            {type: 'numberLiteral', value: '1'},
            {type: 'numberLiteral', value: '2'},
            {type: 'op', op: '+'},
            {type: 'numberLiteral', value: '3'},
            {type: 'op', op: '+'}
          ],[
            {type: 'numberLiteral', value: '1'},
            {type: 'numberLiteral', value: '2'},
            {type: 'op', op: '*'},
            {type: 'numberLiteral', value: '3'},
            {type: 'op', op: '*'}
          ]]
        }
      ]);
    });

    it('should parse "current address" symbol', function() {
      const code =
      `
      .db 2*($+1)
      `;
      const ast = osciasm.parse(new osciasm.StringSource(code));
      ast.forEach(i => osciasm.stripPositions(i))

      expect(ast).to.deep.equal([
        {
          type: 'asmInstruction',
          instruction: 'db',
          ops: [[
            {type: 'numberLiteral', value: '2'},
            {type: 'symbol', value: '$'},
            {type: 'numberLiteral', value: '1'},
            {type: 'op', op: '+'},
            {type: 'op', op: '*'}
          ]]
        }
      ]);
    });

    it('should parse labels', function() {
      const code =
      `
      .db someLabel+2
      `;
      const ast = osciasm.parse(new osciasm.StringSource(code));
      ast.forEach(i => osciasm.stripPositions(i))

      expect(ast).to.deep.equal([
        {
          type: 'asmInstruction',
          instruction: 'db',
          ops: [[
            {type: 'symbol', value: 'someLabel'},
            {type: 'numberLiteral', value: '2'},
            {type: 'op', op: '+'}
          ]]
        }
      ]);
    });
    it('should parse negative numbers', function() {
      const code =
      `
      .db -1 + 4
      `;
      const ast = osciasm.parse(new osciasm.StringSource(code));
      ast.forEach(i => osciasm.stripPositions(i))

      expect(ast).to.deep.equal([
        {
          type: 'asmInstruction',
          instruction: 'db',
          ops: [[
            {type: 'numberLiteral', value: '1'},
            {type: 'op', op: 'unary-'},
            {type: 'numberLiteral', value: '4'},
            {type: 'op', op: '+'}
          ]]
        }
      ]);
    });

    it('should parse hexadecimal, octal and binary literals', function() {
      const code =
      `
      .db 0xFF - 0777 + 0b111 ; With spaces!
      `;
      const ast = osciasm.parse(new osciasm.StringSource(code));
      ast.forEach(i => osciasm.stripPositions(i))

      expect(ast).to.deep.equal([
        {
          type: 'asmInstruction',
          instruction: 'db',
          ops: [[
            {type: 'numberLiteral', value: '0xFF'},
            {type: 'numberLiteral', value: '0777'},
            {type: 'op', op: '-'},
            {type: 'numberLiteral', value: '0b111'},
            {type: 'op', op: '+'}
          ]]
        }
      ]);
    });

    it('should parse string literals', function() {
      const code =
      `
      .db "Lets try a string"
      `;
      const ast = osciasm.parse(new osciasm.StringSource(code));
      ast.forEach(i => osciasm.stripPositions(i))

      expect(ast).to.deep.equal([
        {
          type: 'asmInstruction',
          instruction: 'db',
          ops: [[{
            type: 'stringLiteral',
            value: 'Lets try a string'
          }]]
        }
      ]);
    });

    it('should parse complex instructions', function() {
      const code =
      `
      loop: someStringLabel+4*someCounter someOtherString+someCounter register0 $+4*12
      `;
      const ast = osciasm.parse(new osciasm.StringSource(code));
      ast.forEach(i => osciasm.stripPositions(i))

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

  describe('assemble', function() {
    it('should increment IP appropriately', function() {
      const code =
      `
      1 2 3 $+4*4
      1 2 3 $+128
      `;

      const asm = osciasm.assemble(osciasm.parse(new osciasm.StringSource(code)));
      expect(asm).to.deep.equal([
        1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 16,     0, 0, 0,
        1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 16+128, 0, 0, 0
      ]);
    });

    it('should make .db and .dw multiples of word size', function() {
      const code =
      `
      .db 1 2 ; just 2 bytes
      .dw 1 2 ; just 2 words
      `;

      const asm = osciasm.assemble(osciasm.parse(new osciasm.StringSource(code)));
      expect(asm).to.deep.equal([
        1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
      ]);
    });

    it('should make jmp targets a multiple of word size§', function() {
      const code =
      `
      0 0 0 1
      `;

      const asm = osciasm.assemble(osciasm.parse(new osciasm.StringSource(code)));
      expect(asm).to.deep.equal([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0
      ]);
    });

    it('should handle references to labels', function() {
      const code =
      `
      1 2 3 target
      0 0 0 0
      target: 1 2 3 16
      `;

      const asm = osciasm.assemble(osciasm.parse(new osciasm.StringSource(code)));
      expect(asm).to.deep.equal([
        1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 32, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 16, 0, 0, 0
      ]);
    });

    it('should respect .addr for forward references', function() {
      const code =
      `
      1 2 3 target
      .addr 0x80
      target: 1 2 3 0
      `;

      const asm = osciasm.assemble(osciasm.parse(new osciasm.StringSource(code)));
      expect(asm).to.deep.equal([
        1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 128, 0, 0, 0,
        1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0,   0, 0, 0
      ]);
    });
  });

  describe('assembleInstruction', function() {
    it('should turn instructions into little-endian byte arrays', function() {
      const instr = {
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
          value: '16'
        }]
      };
      expect(osciasm.assembleInstruction(instr, osciasm.defaultStartState())).
        to.deep.equal([1,0,0,0,2,0,0,0,3,0,0,0,16,0,0,0]);
    });

    it('should turn .db instructions into little-endian byte arrays', function() {
      const instr = [
        {
          type: 'asmInstruction',
          instruction: 'db',
          ops: [[{
            type: 'numberLiteral',
            value: '1'
          }], [{
            type: 'numberLiteral',
            value: '257'
          }], [{
            type: 'numberLiteral',
            value: '0'
          }]]
        }
      ];

      expect(osciasm.assemble(instr, osciasm.defaultStartState())).
        to.deep.equal([1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    });

    it('should handle negative values for .db, .dw', function() {
      const instr = [
        {
          type: 'asmInstruction',
          instruction: 'db',
          ops: [[{
            type: 'numberLiteral',
            value: '1'
          }, {
            type: 'op',
            op: 'unary-'
          }]]
        }, {
          type: 'asmInstruction',
          instruction: 'dw',
          ops: [[{
            type: 'numberLiteral',
            value: '1'
          }, {
            type: 'op',
            op: 'unary-'
          }]]
        }
      ];

      expect(osciasm.assemble(instr, osciasm.defaultStartState())).
        to.deep.equal([0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                       0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    });

    it('should turn .dw instructions into little-endian byte arrays', function() {
      const instr = [
        {
          type: 'asmInstruction',
          instruction: 'dw',
          ops: [[{
            type: 'numberLiteral',
            value: '1'
          }], [{
            type: 'numberLiteral',
            value: '0xFFAA0033'
          }], [{
            type: 'numberLiteral',
            value: '0'
          }]]
        }
      ];

      expect(osciasm.assemble(instr, osciasm.defaultStartState())).
        to.deep.equal([1, 0, 0, 0, 0x33, 0, 0xAA, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0]);
    });
  });


  describe('StringSource', function() {
    it('should return endOfSource on empty string', function() {
      const source = new osciasm.StringSource('');
      expect(source.peek()).to.equal(osciasm.endOfSource);
      expect(source.pop()).to.equal(osciasm.endOfSource);
    });

    it('should throw when consuming beyond endOfSource', function() {
      const source = new osciasm.StringSource('');
      source.pop();
      expect(source.peek).to.throw();
      expect(source.pop).to.throw();
    });

    it('should yield characters of a string', function() {
      const source = new osciasm.StringSource('1234');
      expect(source.peek()).to.equal('1');
      expect(source.pop()).to.equal('1');
      expect(source.peek()).to.equal('2');
      expect(source.pop()).to.equal('2');
      expect(source.peek()).to.equal('3');
      expect(source.pop()).to.equal('3');
      expect(source.peek()).to.equal('4');
      expect(source.pop()).to.equal('4');
      expect(source.peek()).to.equal(osciasm.endOfSource);
      expect(source.pop()).to.equal(osciasm.endOfSource);
      expect(source.peek).to.throw();
      expect(source.pop).to.throw();
    });

    it('should give the correct position', function() {
      const source =
        new osciasm.StringSource('1234\n1234', {line: 0, character: 0, filename: 'test.txt'});
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
      const source = new osciasm.ConcatSource();
      expect(source.peek()).to.equal(osciasm.endOfSource);
      expect(source.pop()).to.equal(osciasm.endOfSource);
    });

    it('should throw when consuming beyond endOfSource', function() {
      const source = new osciasm.ConcatSource();
      source.pop();
      expect(source.peek).to.throw();
      expect(source.pop).to.throw();
    });

    it('should yield characters of a string', function() {
      const source = new osciasm.ConcatSource(
        new osciasm.StringSource('12'),
        new osciasm.StringSource('3'),
        new osciasm.StringSource('4'));
      expect(source.peek()).to.equal('1');
      expect(source.pop()).to.equal('1');
      expect(source.peek()).to.equal('2');
      expect(source.pop()).to.equal('2');
      expect(source.peek()).to.equal('3');
      expect(source.pop()).to.equal('3');
      expect(source.peek()).to.equal('4');
      expect(source.pop()).to.equal('4');
      expect(source.peek()).to.equal(osciasm.endOfSource);
      expect(source.pop()).to.equal(osciasm.endOfSource);
      expect(source.peek).to.throw();
      expect(source.pop).to.throw();
    });

    it('should give the correct position', function() {
      const source = new osciasm.ConcatSource(
        new osciasm.StringSource('12', {line: 0, character: 0, filename: 'a.txt'}),
        new osciasm.StringSource('3', {line: 0, character: 0, filename: 'b.txt'}),
        new osciasm.StringSource('4', {line: 0, character: 0, filename: 'c.txt'})
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
      const source = new osciasm.ConcatSource(
        new osciasm.StringSource(''),
        new osciasm.StringSource('A')
      );

      expect(source.peek()).to.equal('A');
      expect(source.pop()).to.equal('A');
    });
  });

  describe('tokenizeExpression', function() {
    it('should tokenize expressions with parenthesis', function() {
      const ast = osciasm.parseInstruction(new osciasm.StringSource('.db 2*3*($+1+3)')).ops[0];
      const tokenized = osciasm.tokenizeExpression(ast);
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
      const ast = osciasm.parseInstruction(new osciasm.StringSource('.db 2+3*1')).ops[0];
      const tokenized = osciasm.tokenizeExpression(ast);
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
      const ast = osciasm.parseInstruction(new osciasm.StringSource('.db 2+3*1')).ops[0];
      const tokenized = osciasm.tokenizeExpression(ast);
      const rpn = osciasm.buildRPN(tokenized);
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

  describe('evaluateRPN', function() {
    it('calcuates numbers', function() {
      const expr = [{
          type: 'numberLiteral',
          value: '0xf' // 15
        },
        {
          type: 'numberLiteral',
          value: '010' // 8
        },
        {
          type: 'numberLiteral',
          value: '0b101' // 5
        },
        {
          type: 'numberLiteral',
          value: '10'
        },
        {
          type: 'op',
          op: '*'
        },
        {
          type: 'op',
          op: '+'
        },
        {
          type: 'op',
          op: '+'
        }];
      expect(osciasm.evaluateRPN(expr, {})).to.equal(73);
    });

    it('evaluates symbols', function() {
      const expr = [{
          type: 'numberLiteral',
          value: '2'
        },
        {
          type: 'numberLiteral',
          value: '3'
        },
        {
          type: 'symbol',
          value: 'lol'
        },
        {
          type: 'op',
          op: '*'
        },
        {
          type: 'op',
          op: '+'
        }];
      expect(osciasm.evaluateRPN(expr, {lol: 4})).to.equal(14);
    });
  });


});