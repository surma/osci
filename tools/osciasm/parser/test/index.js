const chai = require('chai');
const expect = chai.expect;

const parser = require('../');

describe('Parser', function() {
  it('should have a parsing function', function() {
    expect(parser).to.have.property('parse');
  })

  it('should ignore comments', function() {
    const code =
    `
    ; This is a comment
    `
    expect(parser.parse(new parser.StringSource(code))).to.have.length(0);
  })

  it.only('should parse instructions', function() {
    const code =
    `
    1 2 3 4
    4 3 2 1
    `
    const ast = parser.parse(new parser.StringSource(code));
    ast.forEach(i => parser.stripPositions(i))
    expect(ast).to.have.length(2);
    expect(ast).to.deep.equal([
      {
        type: 'cpuInstruction',
        label: null,
        operandA: {
          type: 'numberLiteral',
          value: '1'
        },
        operandB: {
          type: 'numberLiteral',
          value: '2'
        },
        target: {
          type: 'numberLiteral',
          value: '3'
        },
        jump: {
          type: 'numberLiteral',
          value: '4'
        }
      },
      {
        type: 'cpuInstruction',
        label: null,
        operandA: {
          type: 'numberLiteral',
          value: '4'
        },
        operandB: {
          type: 'numberLiteral',
          value: '3'
        },
        target: {
          type: 'numberLiteral',
          value: '2'
        },
        jump: {
          type: 'numberLiteral',
          value: '1'
        }
      }
    ]);
  })

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
  })
});
