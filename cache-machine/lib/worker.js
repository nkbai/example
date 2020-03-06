
var set = new Set();
const { parentPort } = require('worker_threads');
onmessage = function (e) {
    // console.log('Message received from main script');
    console.log("cccc");
    // console.log('Posting message back to main script');
    set.add(e.data[0]);
    postMessage(set);
}
parentPort.once('message', message => {
    console.log('message',message);
    set.add(message);
    parentPort.postMessage({ pong: message })
});  