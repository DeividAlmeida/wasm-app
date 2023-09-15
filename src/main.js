const { wasm_func } = require('../pkg')
const js_func = require('./js_func')
const data = require('./data.json')
  
  console.log('Starting!');

  console.time('js-time');
  for (let i = 0; i < data.length; i++) {
    let res = js_func(
      -23.429435,
      -46.39188, 
      data[i].latitude, 
      data[i].longitude
    );
    console.log(res);
  }
  console.timeEnd('js-time');

  console.time('wasm-time');
  for (let i = 0; i < data.length; i++) {
    let res = wasm_func(
      -23.429435,
      -46.39188, 
      data[i].latitude, 
      data[i].longitude
    );
    console.log(res);
  }
  console.timeEnd('wasm-time');



//https://medium.com/pagarme/webassembly-na-pr%C3%A1tica-utilizando-node-js-e-rust-df3c627235bd