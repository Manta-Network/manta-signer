const WebSocket = require('ws').WebSocket;
var recparams = require('./recparams.json');

var client = new WebSocket('ws://localhost:29987/ws');

client.on('error', function() {
    console.log('Connection Error');
});

client.on('open', function() {
    console.log('WebSocket Client Connected');

    function sendNumber() {
        if (client.readyState === client.OPEN) {
            //client.send(JSON.stringify("version"));
            client.send(JSON.stringify({"recover-account": recparams}));
            setTimeout(sendNumber, 1000);
        }
    }
    sendNumber();
});

client.on('close', function() {
    console.log('echo-protocol Client Closed');
});

client.on('message', function(e) {
    console.log("Got a thing");
    console.log(`${e}`);
});
