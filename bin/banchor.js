#!/usr/bin/env node
const { spawnSync } = require('node:child_process');
const path = require('node:path');
const os = require('node:os');
const fs = require('node:fs');
const candidates = [
  path.join(os.homedir(), '.cargo', 'bin', 'banchor'),
  '/usr/local/bin/banchor',
  'banchor',
];
let exe = candidates.find((p) => p === 'banchor' || (fs.existsSync(p) && fs.statSync(p).isFile())) || 'banchor';
const result = spawnSync(exe, process.argv.slice(2), { stdio: 'inherit' });
process.exit(result.status ?? 1);
