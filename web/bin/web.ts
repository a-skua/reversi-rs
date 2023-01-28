import init from "../wasm/wasm.js";
import app from "../src/app.ts";

init().then(() => {
  app();
});
