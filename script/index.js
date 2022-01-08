import { LCDClient, MsgStoreCode, MnemonicKey, isTxError } from '@terra-money/terra.js';
import * as fs from 'fs';

// connect to localterra
const terra = new LCDClient({
  URL: 'http://localhost:1317',
  chainID: 'localterra'
});

const result = await terra.wasm.contractQuery(
  "terra1sh36qn08g4cqg685cfzmyxqv2952q6r8gpczrt",
  {"get_count": {} }
);

console.log(result)