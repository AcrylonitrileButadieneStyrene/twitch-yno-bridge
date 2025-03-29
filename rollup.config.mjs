import terser from "@rollup/plugin-terser";
import typescript from "@rollup/plugin-typescript";

export default {
    input: "./script/index.ts",
    output: {
        file: "script.js",
        format: "iife",
    },
    plugins: [typescript(), terser()],
};
