var addon = require('../native');
const cluster = require('cluster');
console.log('=====>')
console.log(cluster.isMaster);