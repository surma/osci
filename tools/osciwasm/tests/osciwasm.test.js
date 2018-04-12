import osciwasm from '/base/tools/osciwasm/src/osciwasm.js';
import {modulePath} from '/base/tools/osciwasm/tests/helpers/constants.js';

describe('(basic) osciwasm', function () {
  it('should be a function', function () {
    expect(typeof osciwasm).to.equal('function');
  });

  it('should load the wasm module', async function () {
    const x = await osciwasm(modulePath());
  });
});
