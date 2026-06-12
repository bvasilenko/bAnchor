#!/usr/bin/env node
const { spawnSync } = require('node:child_process');
if (process.env.SKIP_BSUITE_INSTALL === '1') process.exit(0);
const cargo = spawnSync('cargo', ['--version'], { stdio: 'ignore' });
if (cargo.status !== 0) {
  console.warn('@booga/banchor: cargo not found; skipping native install. Install Rust + run: cargo install --git https://github.com/bvasilenko/bAnchor');
  process.exit(0);
}
const install = spawnSync('cargo', ['install', '--git', 'https://github.com/bvasilenko/bAnchor', '--locked', '--force', 'banchor'], { stdio: 'inherit' });
process.exit(install.status ?? 0);
