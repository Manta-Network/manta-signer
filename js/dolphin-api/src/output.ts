import { DolphinTypes } from '.';
var fs = require('fs');
var json = JSON.stringify(DolphinTypes, null, 2);
fs.writeFile('types.json', json, 'utf8', function(err) {
    if (err) throw err;
    console.log('complete');
    });