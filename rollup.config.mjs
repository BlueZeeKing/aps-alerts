import { nodeResolve } from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import json from "@rollup/plugin-json";

export default {
  input: "main.js",
  output: {
    file: "out/bundle.js",
    format: "cjs",
  },
  plugins: [nodeResolve(), commonjs(), json()],
};
