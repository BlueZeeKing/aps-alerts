import { nodeResolve } from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import json from "@rollup/plugin-json";
import { defineConfig } from "rollup";

export default defineConfig({
  input: "out/index.js",
  output: {
    dir: "out",
    format: "cjs",
    manualChunks: {
      deps: ["axios"],
    },
  },
  plugins: [nodeResolve(), commonjs(), json()],
});
