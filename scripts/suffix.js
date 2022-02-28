const fs = require('fs');
var cfgdata = JSON.parse(fs.readFileSync('./ui/src-tauri/tauri.conf.json'));
cfgdata.package.version = process.argv[2];
const out = JSON.stringify(cfgdata);
fs.writeFile('./ui/src-tauri/tauri.conf.json', out, (e) => console.log(e));
