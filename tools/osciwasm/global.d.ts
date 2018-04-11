declare namespace WebAssembly {
  function instantiateStreaming(streamSource: Promise<Response> | Response | ReadableStream, importObject?: any): Promise<{module: WebAssembly.Module, instance: WebAssembly.Instance}>;
}
