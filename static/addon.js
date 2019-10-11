WabtModule().then(wabt => {
  window.execute = text =>{
    const mem = new WebAssembly.Memory({ initial: 256 })
    let output = '';
    window.fd_write = (fd, pbuf, iovs_len) => {
      if(fd !== 1) return;

      console.log(`Args: ${fd}, ${pbuf}, ${iovs_len}`);
      console.log(new DataView(mem.buffer, 0, 256));
      const view = new DataView(mem.buffer);
      const buf = view.getUint32(pbuf, true);
      console.log("Buf at: ", buf);
      const buf_len = view.getUint32(pbuf + 4, true);

      const result = [];

      for(let i = 0; i < buf_len && i < iovs_len; ++i) {
        const byte = view.getUint8(buf + i);
        if(byte === 0) break;
        result.push(byte);
      }

      console.log(result);

      output += result.map(e => String.fromCharCode(e)).join();
    };

    const injected = text.replace("(memory 1)",`(memory (import "js" "mem") 1)`);
    console.log(injected);

    const mod = wabt.parseWat('test.wast', injected);
    mod.resolveNames();
    mod.validate();

    const { buffer: bin, log } = mod.toBinary({ log: true, write_debug_names: true });

    const wasmMod = new WebAssembly.Module(bin);
    const inst = new WebAssembly.Instance(wasmMod, {
      wasi_unstable: {
        fd_write
      },
      js: {
        mem,
      },
    });

    const { _start } = inst.exports;

    _start();

    return output;
  };
});
