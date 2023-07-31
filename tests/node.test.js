// autocorrect: false
const assert = require('assert');
const path = require('path');
const autocorrect = require('../autocorrect-node');

const workDir = path.resolve(__dirname, '..');
const ignore = new autocorrect.Ignorer(workDir);

assert.equal(autocorrect.format('Hello你好.'), 'Hello 你好。');
