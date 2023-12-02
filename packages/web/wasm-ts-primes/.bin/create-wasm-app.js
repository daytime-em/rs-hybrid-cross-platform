#!/usr/bin/env node

const { spawn } = require("child_process");
const fs = require("fs");
const process = require("process");

let folderName = '.';

if (process.argv.length >= 3) {
  folderName = process.argv[2];
  if (!fs.existsSync(folderName)) {
    fs.mkdirSync(folderName);
  }
}

console.warn("My working dir", process.cwd());

const clone = spawn("git", ["clone", "https://github.com/rustwasm/create-wasm-app.git", folderName]);

clone.on("close", code => {
  if (code !== 0) {
    console.error("cloning the template failed!")
    process.exit(code);
  } else {
    console.log("🦀 Rust + 🕸 Wasm = ❤");
  }
});
