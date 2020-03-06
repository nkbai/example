console.log('Child initializing..');

process.on('message', function(message) {
    console.log('message from master: ', message);
});

process.send('Hello from Child!');