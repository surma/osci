function getStrFromMemory(memory: WebAssembly.Memory, addr: number, size: number): string
 {
  const strBuf = new Uint8Array(memory.buffer, addr, size);
  return new TextDecoder().decode(strBuf);
}

function putArrayIntoMemory(data: ArrayBuffer, memory: WebAssembly.Memory, addr: number, len: number) {
  if(data.byteLength !== len) {
    throw new Error("Data must have exact lenght of slice (for now)");
  }
  // new Uint8Array(memory.buffer).set(new Uint8Array(data), addr);
}

export default async function (path = 'osciasm.wasm') {
  let instance: WebAssembly.Instance;

  const importObj = {
    env: {
      _js_print: (addr: number, size: number) => {
        console.log(getStrFromMemory(instance.exports.memory, addr, size));
      },
    },
  };
  instance = (await WebAssembly.instantiateStreaming(
    fetch(path), importObj
  )).instance;
  return {
    loader: {
      hexloader: {
        load(data: string): number {
          const buf = new TextEncoder().encode(data);
          const sliceAddr = instance.exports.wasm__allocate_u8_slice(buf.byteLength);
          putArrayIntoMemory(buf.buffer, instance.exports.memory, sliceAddr, buf.byteLength);
          return instance.exports.loader__hexloader__load(sliceAddr);
        },
      },
    },
  };
};
