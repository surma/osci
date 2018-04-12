export function modulePath(debug = false) {
  return `/base/target/wasm32-unknown-unknown/${debug ? 'debug': 'release'}/examples/osciwasm.wasm`;
}
