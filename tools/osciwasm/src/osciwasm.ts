interface FatPointer {
    addr: number,
    size: number
};

function getStr(instance: WebAssembly.Instance, str: FatPointer): string
 {
  const strBuf = new Uint8Array(instance.exports.memory.buffer, str.addr, str.size);
  return new TextDecoder().decode(strBuf);
}

function loadU8Buffer(data: Uint8Array, instance: WebAssembly.Instance): FatPointer {
  const sliceAddr = instance.exports.wasm__allocate_u8_slice(data.byteLength, 0);
  const slicePtrAddr = instance.exports.wasm__get_u8_slice_data_ptr(sliceAddr, data.byteLength);
  const slice = {addr: slicePtrAddr, size: data.byteLength};
  new Uint8Array(instance.exports.memory.buffer).set(data, slice.addr);
  return slice;
}

function loadString(str: string, instance: WebAssembly.Instance): FatPointer {
    const buf = new TextEncoder().encode(str);
    return loadU8Buffer(buf, instance);
}


export default async function (path = 'osciasm.wasm') {
  let instance: WebAssembly.Instance;

  const importObj = {
    env: {
      _js_print: (addr: number, size: number) => {
        console.log(getStr(instance, {addr, size}));
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
