WabtModule().then(wabt => {
  window.execute = text => {
    let output = '';
    window.fd_write = (fd, pbuf, iovs_len) => {
      if (fd !== 1) return; // stdout
      if (window.wasmMemory === undefined) return;

      const view = new DataView(window.wasmMemory.buffer);

      const result = [];

      for (let i = 0; i < iovs_len; i++) {
        const buf = view.getUint32(pbuf + i * 8, true);
        const buf_len = view.getUint32(pbuf + i * 8 + 4, true);
        for (let j = 0; j < buf_len; j++) {
          const byte = view.getUint8(buf + j);
          result.push(byte);
        }
      }

      output += result.map(e => String.fromCharCode(e)).join();
    };

    const mod = wabt.parseWat('test.wast', text);
    mod.resolveNames();
    mod.validate();

    const { buffer: bin, log } = mod.toBinary({ log: true, write_debug_names: true });

    const wasmMod = new WebAssembly.Module(bin);
    const inst = new WebAssembly.Instance(wasmMod, {
      wasi_unstable: {
        fd_write
      }
    });

    const { _start, memory } = inst.exports;

    window.wasmMemory = memory;
    _start();

    return output;
  };
});