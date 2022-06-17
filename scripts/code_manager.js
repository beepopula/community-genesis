// import "regenerator-runtime/runtime.js";
const nearAPI = require("near-api-js");
const getConfig = require("./config.js");
const nearConfig = getConfig("development");
const fs = require('fs');
const js_sha256 = require("js-sha256")
const bs58 = require("bs58")
const GAS = "300000000000000";
const yargs = require("yargs")
const {functionCall} = require("near-api-js/lib/transaction.js")

class Contract {

    account
    contract
  
    constructor(account, contract) {
      this.account = account
      this.contract = contract
    }

    async addCommunityType(type, length, hash) {
      await this.contract.add_code_type({community_type: type, length: length, hash: hash}, GAS, 0)
    }

    async delCommunityType(type) {
      await this.contract.del_code_type({community_type: type})
    }

    async addCode(code) {
      const actions = [functionCall("add_code", code, GAS)];
      await this.account.signAndSendTransaction({receiverId: nearConfig.contractName, actions: actions})
    }



    static async new(accountId) {
      console.log(accountId)
      let keyStore = new nearAPI.keyStores.UnencryptedFileSystemKeyStore("/home/ubuntu/.near-credentials");
  
      const near = await nearAPI.connect({
        keyStore: keyStore,
        ...nearConfig
      });
  
      const account = await near.account(accountId);
  
      // Initializing our contract APIs by contract name and configuration.
      const contract = await new nearAPI.Contract(account, nearConfig.contractName, {
          // View methods are read-only – they don't modify the state, but usually return some value
          viewMethods: [],
          // Change methods can modify the state, but you don't receive the returned value when called
          changeMethods: ['add_code_type', 'del_code_type'],
          // Sender is the account ID to initialize transactions.
          // getAccountId() will return empty string if user is still unauthorized
          sender: account
      });

      return new Contract(account, contract)
    }
  
  }

async function addType(contract, type) {
    let file = fs.readFileSync(`../res/${type}.wasm`)
    let length = file.length
    let hash = bs58.encode(js_sha256.sha256.digest(file))
    contract.addCommunityType(type, length, hash)
    await contract.addCode(file)
}

async function delType(contract, type) {
  await contract.delCommunityType(type)
}

async function checkHash(contract, type) {
  let file = fs.readFileSync(`../res/${type}.wasm`)
  let hash = bs58.encode(js_sha256.sha256.digest(file))
  console.log(hash)
}

//addType()
//delType()
//checkHash()

async function init() {
  yargs
  .scriptName("code manager")
  .usage('$0 <cmd> [args]')
  .options({ 
    accountId: { 
      type: 'string',
      describe: 'account ID',
      alias: 'a', 
      hidden: false,
    }
  })
  .command('set [type]', 'set a community type', (yargs) => {
    yargs.positional('type', {
      type: 'string',
      default: 'normal',
      describe: 'community type'
    })
  }, async function (argv) {
    let contract = await Contract.new(argv.accountId)
    addType(contract, argv.type)
  })
  .command('del [type]', 'del a community type', (yargs) => {
    yargs.positional('type', {
      type: 'string',
      default: 'normal',
      describe: 'community type'
    })
  }, async function (argv) {
    let contract = await Contract.new(argv.accountId)
    delType(contract, argv.type)
  })
  .argv
}

init()