#!/usr/bin/env node
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// Path to the downloaded binary
const binName = process.platform === 'win32' ? 'ccap-windows-x64.exe' : (process.platform === 'darwin' ? 'ccap-macos-x64' : 'ccap-linux-x64');
const binPath = path.join(__dirname, '..', 'binaries', binName);

if (!fs.existsSync(binPath)) {
  console.error(`❌ CCAP binary not found at ${binPath}. Please run 'npm install' again.`);
  process.exit(1);
}

const child = spawn(binPath, process.argv.slice(2), { stdio: 'inherit' });
child.on('exit', (code) => process.exit(code));
