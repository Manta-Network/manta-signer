const WebSocket = require('ws').WebSocket;
var recover_account = require('./recover-account.json');
var derive_shielded_address = require('./derive-shielded-address.json');
var generate_asset = require('./generate-asset.json');
var generate_mint_data = require('./generate-mint-data.json');
var generate_private_transfer_data = require('./generate-private-transfer-data.json');
var generate_reclaim_data = require('./generate-reclaim-data.json');

var client = new WebSocket('ws://localhost:29987/ws');

client.on('error', function() {
    console.log('Connection Error');
});

client.on('open', function() {
    console.log('WebSocket Client Connected');

    function sendNumber() {
        if (client.readyState === client.OPEN) {
            //client.send(JSON.stringify("version"));
            client.send(JSON.stringify({"recover-account": recover_account}));
            //client.send(JSON.stringify({"derive-shielded-address": derive_shielded_address}));
            //client.send(JSON.stringify({"generate-asset": generate_asset}));
            //client.send(JSON.stringify({"generate-mint-data": generate_mint_data}));
            //client.send(JSON.stringify({"generate-private-transfer-data": generate_private_transfer_data}));
            //client.send(JSON.stringify({"generate-reclaim-data": generate_reclaim_data}));
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
