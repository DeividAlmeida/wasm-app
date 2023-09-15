const { MyObject } = require('../pkg')
const fibonacciJs = require('./fibonacci')

const run = async () => {
  const total = 
    [{
      "id": 1,
      "name": "Leanne Graham",
      "username": "Bret",
      "email": "testes@dsa"
    }]
  
  const foo =  new MyObject(total);
  console.log('Starting!');

  console.time('wasm-time');

  console.log(foo.get());
  console.timeEnd('wasm-time');

  // console.time('js-time');
  // console.log(fibonacciJs(total));
  // console.timeEnd('js-time');
}

run()


//https://medium.com/pagarme/webassembly-na-pr%C3%A1tica-utilizando-node-js-e-rust-df3c627235bd