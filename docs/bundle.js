// wasm/wasm.js
var wasm;
var cachedTextDecoder = new TextDecoder("utf-8", { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
var cachedUint8Memory0 = new Uint8Array();
function getUint8Memory0() {
  if (cachedUint8Memory0.byteLength === 0) {
    cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8Memory0;
}
function getStringFromWasm0(ptr, len) {
  return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
var Player = Object.freeze({ Player1: 0, "0": "Player1", Player2: 1, "1": "Player2" });
var State = Object.freeze({ Empty: 0, "0": "Empty", Player1: 1, "1": "Player1", Player2: 2, "2": "Player2" });
var Reversi = class {
  static __wrap(ptr) {
    const obj = Object.create(Reversi.prototype);
    obj.ptr = ptr;
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.ptr;
    this.ptr = 0;
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_reversi_free(ptr);
  }
  /**
  * @returns {Reversi}
  */
  static new_game() {
    const ret = wasm.reversi_new_game();
    return Reversi.__wrap(ret);
  }
  /**
  * @returns {number}
  */
  x() {
    const ret = wasm.reversi_x(this.ptr);
    return ret >>> 0;
  }
  /**
  * @returns {number}
  */
  y() {
    const ret = wasm.reversi_y(this.ptr);
    return ret >>> 0;
  }
  /**
  * @param {number} p
  * @param {number} x
  * @param {number} y
  * @returns {boolean}
  */
  is_act(p, x, y) {
    const ret = wasm.reversi_is_act(this.ptr, p, x, y);
    return ret !== 0;
  }
  /**
  * @param {number} x
  * @param {number} y
  * @returns {number | undefined}
  */
  state(x, y) {
    const ret = wasm.reversi_state(this.ptr, x, y);
    return ret === 3 ? void 0 : ret;
  }
  /**
  * @returns {number}
  */
  current_player() {
    const ret = wasm.reversi_current_player(this.ptr);
    return ret >>> 0;
  }
  /**
  * @param {number} p
  * @param {number} x
  * @param {number} y
  * @returns {Reversi | undefined}
  */
  action(p, x, y) {
    const ret = wasm.reversi_action(this.ptr, p, x, y);
    return ret === 0 ? void 0 : Reversi.__wrap(ret);
  }
};
async function load(module, imports) {
  if (typeof Response === "function" && module instanceof Response) {
    if (typeof WebAssembly.instantiateStreaming === "function") {
      try {
        return await WebAssembly.instantiateStreaming(module, imports);
      } catch (e) {
        if (module.headers.get("Content-Type") != "application/wasm") {
          console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
        } else {
          throw e;
        }
      }
    }
    const bytes = await module.arrayBuffer();
    return await WebAssembly.instantiate(bytes, imports);
  } else {
    const instance = await WebAssembly.instantiate(module, imports);
    if (instance instanceof WebAssembly.Instance) {
      return { instance, module };
    } else {
      return instance;
    }
  }
}
function getImports() {
  const imports = {};
  imports.wbg = {};
  imports.wbg.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
  };
  return imports;
}
function initMemory(imports, maybe_memory) {
}
function finalizeInit(instance, module) {
  wasm = instance.exports;
  init.__wbindgen_wasm_module = module;
  cachedUint8Memory0 = new Uint8Array();
  return wasm;
}
async function init(input) {
  if (typeof input === "undefined") {
    input = new URL("wasm_bg.wasm", import.meta.url);
  }
  const imports = getImports();
  if (typeof input === "string" || typeof Request === "function" && input instanceof Request || typeof URL === "function" && input instanceof URL) {
    input = fetch(input);
  }
  initMemory(imports);
  const { instance, module } = await load(await input, imports);
  return finalizeInit(instance, module);
}
var wasm_default = init;

// src/app.ts
var windowWidth = () => window.innerWidth;
var windowHeight = () => window.innerHeight - 32;
var Board = class {
  constructor(canvas) {
    this.focus = null;
    this.x = 10;
    this.y = 10;
    this.w = canvas.width - 20;
    this.h = canvas.height - 20;
    this.tileW = this.w / 8;
    this.tileH = this.h / 8;
  }
  position(offsetX, offsetY) {
    return {
      x: Math.floor(offsetX / this.tileW),
      y: Math.floor(offsetY / this.tileH)
    };
  }
  draw(ctx) {
    if (this.focus !== null) {
      ctx.fillStyle = "grey";
      ctx.fillRect(
        this.x + this.focus.x * this.tileW,
        this.y + this.focus.y * this.tileH,
        this.tileW,
        this.tileH
      );
      ctx.fillStyle = "black";
    }
    ctx.strokeRect(this.x, this.y, this.w, this.h);
    ctx.beginPath();
    for (let n = 1; n < 8; n++) {
      ctx.moveTo(this.x + this.tileW * n, this.y);
      ctx.lineTo(this.x + this.tileW * n, this.y + this.h);
      ctx.moveTo(this.x, this.y + this.tileH * n);
      ctx.lineTo(this.x + this.w, this.y + this.tileH * n);
    }
    ctx.stroke();
  }
};
var Tile = class {
  constructor(board, state, x, y) {
    this.board = board;
    this.state = state;
    this.x = x;
    this.y = y;
  }
  draw(ctx) {
    const x = this.board.x + this.board.tileW * this.x + this.board.tileW / 2;
    const y = this.board.y + this.board.tileH * this.y + this.board.tileH / 2;
    const radius = this.board.tileW * 0.4;
    switch (this.state) {
      case State.Player1:
        ctx.beginPath();
        ctx.arc(x, y, radius, 0, Math.PI * 2);
        ctx.fill();
        break;
      case State.Player2:
        ctx.beginPath();
        ctx.arc(x, y, radius, 0, Math.PI * 2);
        ctx.stroke();
        break;
    }
  }
};
function resizeElement(elem) {
  const w = windowWidth();
  const h = windowHeight();
  const size = w < h ? w : h;
  elem.width = size;
  elem.height = size;
}
function draw(board, ctx, game) {
  const draw2 = [board];
  for (let y = 0; y < game.y(); y++) {
    for (let x = 0; x < game.x(); x++) {
      const state = game.state(x, y);
      if (state === void 0) {
        continue;
      }
      draw2.push(new Tile(board, state, x, y));
    }
  }
  for (const obj of draw2) {
    obj.draw(ctx);
  }
}
function app() {
  let game = Reversi.new_game();
  const canvas = document.createElement("canvas");
  resizeElement(canvas);
  let board = new Board(canvas);
  const ctx = canvas.getContext("2d");
  draw(board, ctx, game);
  addEventListener("resize", (_) => {
    resizeElement(canvas);
    board = new Board(canvas);
    draw(board, ctx, game);
  });
  let currentPosition = { x: 0, y: 0 };
  canvas.addEventListener("mousemove", (e) => {
    const newPosition = board.position(e.offsetX, e.offsetY);
    if (currentPosition.x === newPosition.x && currentPosition.y === newPosition.y) {
      return;
    }
    currentPosition = newPosition;
    if (game.is_act(game.current_player(), currentPosition.x, currentPosition.y)) {
      board.focus = currentPosition;
    } else {
      board.focus = null;
    }
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    draw(board, ctx, game);
  });
  canvas.addEventListener("mouseup", (e) => {
    const position = board.position(e.offsetX, e.offsetY);
    const newGame = game.action(game.current_player(), position.x, position.y);
    if (newGame === void 0) {
      return;
    }
    game = newGame;
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    draw(board, ctx, game);
  });
  document.getElementById("app")?.appendChild(canvas);
}
var app_default = app;

// bin/web.ts
wasm_default().then(() => {
  app_default();
});
