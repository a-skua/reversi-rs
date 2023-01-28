import * as esbuild from "https://deno.land/x/esbuild@v0.17.4/mod.js";

await esbuild.build({
  entryPoints: [
    "./bin/web.ts",
  ],
  outfile: "../docs/bundle.js",
  format: "esm",
  bundle: true,
});

esbuild.stop();
