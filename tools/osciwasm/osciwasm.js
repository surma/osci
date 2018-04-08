function get_str_from_memory(memory, addr, size) {
  const strBuf = new Uint8Array(importObj.instance.exports.memory.buffer, addr, size);
  return new TextDecoder().decode(strBuf);
}

export default async function (path = 'osciasm.wasm') {
  let instance;
  const importObj = {
    env: {
      _js_print: (addr, size) => {
        console.log(
          get_str_from_memory(instance.exports.memory, addr, size)
        );
      },
    },
  };
  instance = (await WebAssembly.instantiateStreaming(
    fetch(path), importObj
  )).instance;
  return {
    do_it() {
      return instance.exports.do_it();
    }
  }
};
