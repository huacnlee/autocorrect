// autocorrect: false
const assert = require('assert');
const autocorrect = require('../node-pkg');

assert.equal(autocorrect.format('Hello你好'), 'Hello 你好');
