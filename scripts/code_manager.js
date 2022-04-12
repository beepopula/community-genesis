// import "regenerator-runtime/runtime.js";
const nearAPI = require("near-api-js");
const getConfig = require("./config.js");
const nearConfig = getConfig("development");
const fs = require('fs');
const sha256 = require("crypto-js/sha256")
const base58 = require("base58")
const GAS = "300000000000000";

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
      this.contract = await new nearAPI.Contract(this.account, nearConfig.contractName, {
          // View methods are read-only – they don't modify the state, but usually return some value
          viewMethods: [],
          // Change methods can modify the state, but you don't receive the returned value when called
          changeMethods: ['add_code_type', 'del_code_type'],
          // Sender is the account ID to initialize transactions.
          // getAccountId() will return empty string if user is still unauthorized
          sender: this.account
      });
      this.provider = await new nearAPI.providers.JsonRpcProvider(nearConfig.nodeUrl);
    }

    async addCommunityType(type, length, hash) {
      await this.contract.add_code_type({community_type: type, length: length, hash: hash}, GAS, 0)
    }

    async delCommunityType(type) {
      await this.contract.del_code_type({community_type: type})
    }
  
  }

async function addType() {
    let contract = new Contract()
    await contract.init()
    let file = fs.readFileSync("../res/normal_community.wasm")
    let length = file.length
    let hash = sha256(file).toString()
    contract.addCommunityType("normal", length, hash)
}

async function delType() {
  let contract = new Contract()
  await contract.init()
  await contract.delCommunityType("normal")
}

addType()
//delType()
//del()