interface FatPointer {
    addr: number,
    size: number
};

function getStrFromMemory(memory: WebAssembly.Memory, addr: number, size: number): string
 {
  const strBuf = new Uint8Array(memory.buffer, addr, size);
  return new TextDecoder().decode(strBuf);
}

function copyArrayBufferIntoSlice(data: ArrayBuffer, memory: WebAssembly.Memory, slice: FatPointer) {
  if(data.byteLength !== slice.size) {
    throw new Error("Data must have exact length of slice (for now)");
  }
  new Uint8Array(memory.buffer).set(new Uint8Array(data), slice.addr);
}

function loadString(str: string, instance: WebAssembly.Instance): FatPointer {
    const buf = new TextEncoder().encode(str);
    const sliceAddr = instance.exports.wasm__allocate_u8_slice(buf.byteLength, 0);
    const slicePtrAddr = instance.exports.wasm__get_u8_slice_data_ptr(sliceAddr, buf.byteLength);
    const slice = {addr: slicePtrAddr, size: buf.byteLength};
    copyArrayBufferIntoSlice(buf.buffer, instance.exports.memory, slice);
    return slice;
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
    instance,
    loader: {
      hexloader: {
        load(data: string): number {
          const slice = loadString(data, instance);
          return instance.exports.loader__hexloader__load(slice.addr, slice.size);
        },
      },
    },
  };
};
