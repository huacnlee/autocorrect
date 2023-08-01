#!/usr/bin/env node
const process = require('process');
const autocorrect = require('./index.js');

autocorrect.run(process.argv);
