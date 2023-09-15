const { MyObject } = require('../pkg')
const js_loop = require('./js_func')
const data = require('./data.json')

const run = async () => {
  
  const foo =  new MyObject(data);
  console.log('Starting!');

  console.time('wasm-time');
foo.get();
  console.timeEnd('wasm-time');

  console.time('js-time');
 js_loop(data);
  console.timeEnd('js-time');
}

run()


//https://medium.com/pagarme/webassembly-na-pr%C3%A1tica-utilizando-node-js-e-rust-df3c627235bd