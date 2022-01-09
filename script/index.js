import { LCDClient, MsgInstantiateContract, MsgStoreCode, MnemonicKey, isTxError, Coins} from '@terra-money/terra.js';
import * as fs from 'fs';
import fetch from 'isomorphic-fetch';

// // connect to localterra
// const terra = new LCDClient({
//   URL: 'http://localhost:1317',
//   chainID: 'localterra'
// });

// Fetch gas prices and convert to `Coin` format.
const gasPrices = await (await fetch('https://bombay-fcd.terra.dev/v1/txs/gas_prices')).json();
const gasPricesCoins = new Coins(gasPrices);

// const terra = new LCDClient({
//   URL: "https://bombay-lcd.terra.dev/",
//   chainID: "bombay-12",
//   gasPrices: gasPricesCoins,
//   gasAdjustment: "1.5",
//   gas: 10000000,
// });

// const result = await terra.wasm.contractQuery(
//   "terra1sh36qn08g4cqg685cfzmyxqv2952q6r8gpczrt",
//   {"get_count": {} }
// );

// test1 key from localterra accounts
// const mk = new MnemonicKey({
//   mnemonic: 'popular raven ginger mechanic blind celery uncle will upon tilt midnight cannon wheat issue picture grass either family scheme world salad rice obtain auction'
// })

const mk = new MnemonicKey({
  mnemonic: 'satisfy adjust timber high purchase tuition stool faith fine install that you unaware feed domain license impose boss human eager hat rent enjoy dawn'
})

// connect to localterra
const terra = new LCDClient({
  URL: 'http://localhost:1317',
  chainID: 'localterra'
});

const wallet = terra.wallet(mk);

const storeCode = new MsgStoreCode(
  wallet.key.accAddress,
  fs.readFileSync('./artifacts/my_first_contract-aarch64.wasm').toString('base64')
);
const storeCodeTx = await wallet.createAndSignTx({
  msgs: [storeCode],
});
const storeCodeTxResult = await terra.tx.broadcast(storeCodeTx);

console.log(storeCodeTxResult);

if (isTxError(storeCodeTxResult)) {
  throw new Error(
    `store code failed. code: ${storeCodeTxResult.code}, codespace: ${storeCodeTxResult.codespace}, raw_log: ${storeCodeTxResult.raw_log}`
  );
}

const {
  store_code: { code_id },
} = storeCodeTxResult.logs[0].eventsByType;

console.log(code_id)

console.log(wallet.key.accAddress, 'addresser')

const instantiate = new MsgInstantiateContract(
  wallet.key.accAddress,
  wallet.key.accAddress,
  +code_id[0], // code ID
  {
    receiver: wallet.key.accAddress,
    token: wallet.key.accAddress,
  }, // InitMsg
);

const instantiateTx = await wallet.createAndSignTx({
  msgs: [instantiate],
});
// const instantiateTxResult = await terra.tx.broadcast(instantiateTx);

// console.log(instantiateTxResult);

// if (isTxError(instantiateTxResult)) {
//   throw new Error(
//     `instantiate failed. code: ${instantiateTxResult.code}, codespace: ${instantiateTxResult.codespace}, raw_log: ${instantiateTxResult.raw_log}`
//   );
// }

// const {
//   instantiate_contract: { contract_address },
// } = instantiateTxResult.logs[0].eventsByType;

// console.log(contract_address)