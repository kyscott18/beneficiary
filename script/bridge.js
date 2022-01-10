import { LCDClient, MsgExecuteContract, MnemonicKey, isTxError, Coins} from '@terra-money/terra.js';
import * as fs from 'fs';
import fetch from 'isomorphic-fetch';

// Fetch gas prices and convert to `Coin` format.
const gasPrices = await (await fetch('https://bombay-fcd.terra.dev/v1/txs/gas_prices')).json();
const gasPricesCoins = new Coins(gasPrices);

const terra = new LCDClient({
  URL: "https://bombay-lcd.terra.dev/",
  chainID: "bombay-12",
  gasPrices: gasPricesCoins,
  gasAdjustment: "1.5",
  gas: 10000000,
});

const mk = new MnemonicKey({
  mnemonic: 'popular raven ginger mechanic blind celery uncle will upon tilt midnight cannon wheat issue picture grass either family scheme world salad rice obtain auction'
})

// const mk = new MnemonicKey({
//   mnemonic: 'satisfy adjust timber high purchase tuition stool faith fine install that you unaware feed domain license impose boss human eager hat rent enjoy dawn'
// })

// // connect to localterra
// const terra = new LCDClient({
//   URL: 'http://localhost:1317',
//   chainID: 'localterra'
// });

const wallet = terra.wallet(mk);

const contract = "terra1f2f3z323tpcqq65peyzgdacj9jh37nw6h275m8"

let execute = new MsgExecuteContract(
  wallet.key.accAddress, // sender
  contract, // contract account address
  { 
    approve_bridge: {
      amount: "53",
    } 
  }, // handle msg
);
let executeTx = await wallet.createAndSignTx({
  msgs: [execute]
});

let executeTxResult = await terra.tx.broadcast(executeTx);

execute = new MsgExecuteContract(
  wallet.key.accAddress, // sender
  contract, // contract account address
  { 
    bridge: {
      amount: "53",
      recipient_chain: 1,
      recipient: "4sF2ejiqy2pSm3Vhq5KUD41BTLfeQXHJo7oezDwdmg5J",
      nonce: 4,
    } 
  }, // handle msg
);

executeTx = await wallet.createAndSignTx({
  msgs: [execute]
});

executeTxResult = await terra.tx.broadcast(executeTx);

console.log(executeTxResult)
