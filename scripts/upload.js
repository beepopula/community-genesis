// import "regenerator-runtime/runtime.js";
const nearAPI = require("near-api-js");
const getConfig = require("./config.js");
const nearConfig = getConfig("development");
const fs = require('fs');
const {transaction, functionCall} = require("near-api-js/lib/transaction.js")
const GAS = "300000000000000";
import { functionCall} from "near-api-js/lib/transaction.js";
import { signAndSendTransaction, getTxData } from '../utils/transaction';

class Contract {

    near
    wallet_connection
    contract
    status
    provider
  
    async init() {

      let keyStore = new nearAPI.keyStores.UnencryptedFileSystemKeyStore("/home/ubuntu/.near-credentials");
      
      // const keyStore = new nearAPI.keyStores.UnencryptedFileSystemKeyStore(KEY_PATH);
      // const keyStore = new nearAPI.keyStores.UnencryptedFileSystemKeyStore("/home/bhc/.near-credentials");
      // await keyStore.setKey("testnet", "bhc3.testnet", keyPair);
  
      const near = await nearAPI.connect({
        keyStore: keyStore,
        // keyStore: new nearAPI.keyStores.UnencryptedFileSystemKeyStore("~/.near-credentials/testnet/bhc3.testnet.json"),
        ...nearConfig
      });
  
      this.account = await near.account("bhc8521.testnet");
  
      // Initializing our contract APIs by contract name and configuration.

      this.provider = await new nearAPI.providers.JsonRpcProvider(nearConfig.nodeUrl);
    }

    async addCode(code) {
      const actions = [functionCall("add_code", code, GAS)];
      await this.account.signAndSendTransaction({receiverId: nearConfig.contractName, actions: actions})
    }
  
  }

// function getCodeInfo(path) {
//   let file = fs.readFileSync(path)
//   let code_fragment_length = 10000
//   let code_fragment_count = file.length / code_fragment_length
//   return {
//     code: file,
//     length: file.length,
//     code_fragment_count,
//     code_fragment_length
//   }
// }

// function sleep(ms) {
//   return new Promise(resolve => setTimeout(resolve, ms))
// }

// async function upload() {
//   let contract = new Contract()
//   await contract.init()
//   let code_info = getCodeInfo("../res/normal_community.wasm")
//   let index = 0
//   if (fs.existsSync("index")) {
//     index = Number(fs.readFileSync("index"))
//   }
//   for (let i = index; i <= code_info.code_fragment_count; i ++) {
//     let start = i * code_info.code_fragment_length
//     let end = (i + 1) * code_info.code_fragment_length
//     if (end > code_info.length) {
//       end = code_info.length
//     }
//     fs.writeFileSync("index", String(i))
//     console.log("uploading...", start, "/", code_info.length)
//     //console.log(code_info.code.slice(start, end))
//     let code = code_info.code.slice(start, end)
//     let arr = [...code]
//     await contract.addCode("normal", arr)
//     sleep(1000)
//   }
//   fs.writeFileSync("index", '0')
// }

async function upload() {
  let contract = new Contract()
  await contract.init()
  let file = fs.readFileSync("../res/normal_community.wasm")
  await contract.addCode(file)
}

upload()
