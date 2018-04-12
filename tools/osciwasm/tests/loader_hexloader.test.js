import osciwasm from '/base/tools/osciwasm/src/osciwasm.js';
import {modulePath} from '/base/tools/osciwasm/tests/helpers/constants.js';

describe('osciwasm.loader.hexloader', function () {
  beforeEach(async function () {
    this.osciwasm = await osciwasm(modulePath());

  });

  it('should be defined', function () {
    expect(this.osciwasm.loader).to.exist;
    expect(this.osciwasm.loader.hexloader).to.exist;
    expect(typeof this.osciwasm.loader.hexloader.load).to.be.equal('function');
  });

  it('should parse a simple program', async function () {
    const program = `
      1 2 3 4
    `;
    const memory = this.osciwasm.loader.hexloader.load(program);
    expect(memory).to.not.equal(0);
  });

  it('should fail on a invalid program', async function () {
    const program = `
      lol
    `;
    const memory = this.osciwasm.loader.hexloader.load(program);
    expect(memory).to.equal(0);
  });
});
