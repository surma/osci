import osciwasm from '/base/tools/osciwasm/src/osciwasm.js';
import {modulePath} from '/base/tools/osciwasm/tests/helpers/constants.js';

describe('osciwasm.wasm', function () {
  beforeEach(async function () {
    this.osciwasm = await osciwasm(modulePath());
  });

  it('an allocate slices correctly', function () {
    const slicePtr = this.osciwasm.instance.exports.wasm__allocate_u8_slice(4, 1);
    expect(slicePtr).to.not.equal(0);
    const sliceDataPtr = this.osciwasm.instance.exports.wasm__get_u8_slice_data_ptr(slicePtr, 4);
    expect(sliceDataPtr).to.not.equal(0);
    const memBuffer = this.osciwasm.instance.exports.memory.buffer;
    const sliceData = new Uint8Array(memBuffer, sliceDataPtr, 4);
    expect(Array.from(sliceData)).to.deep.equal([1, 1, 1, 1]);
  });
});
