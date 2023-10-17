const rp = require('request-promise')
// import "regenerator-runtime/runtime.js";
const nearAPI = require("near-api-js");
const getConfig = require("./config.js");
const nearConfig = getConfig("testnet");
const fs = require('fs');
const js_sha256 = require("js-sha256")
const bs58 = require("bs58")
const GAS = "300000000000000";
const yargs = require("yargs")
const {functionCall} = require("near-api-js/lib/transaction.js")

const homedir = require('os').homedir();
const path = require('path');
const CREDENTIALS_DIR = '.near-credentials';
const credentialsPath = path.join(homedir, CREDENTIALS_DIR);

class Contract {
    mainId
    mainSignerKeyPair
    signerKeyPair
    near
  
    constructor(mainId, mainSignerKeyPair, signerKeyPair, near) {
      this.mainId = mainId
      this.mainSignerKeyPair = mainSignerKeyPair
      this.signerKeyPair = signerKeyPair
      this.near = near
    }

    async upgrade(code, contractId, migrate = false) {
      if (contractId != this.mainId) {
        await this.near.config.keyStore.setKey(nearConfig.networkId, contractId, this.signerKeyPair)
      } else {
        await this.near.config.keyStore.setKey(nearConfig.networkId, contractId, this.mainSignerKeyPair)
      }
      const account = await this.near.account(contractId)
      try {
        await account.deployContract(code)
      } catch (e) {
        console.log(e)
        console.log("upgrade error: ", contractId)
        return
      }
        
        
      if (migrate) {
        try {
          await account.functionCall({
            contractId,
            methodName: "migrate", 
            args: {}, 
            gas: GAS, 
            attachedDeposit: 0
          })
          // await account.functionCall({
          //   contractId,
          //   methodName: "set_args",
          //   args: {
          //     args: {
          //       drip_contract: 'v2-drip.beepopula.testnet',
          //     }
          //   },
          //   gas: GAS, 
          //   attachedDeposit: 1
          // })
        } catch (e) {
          console.log(e)
          console.log("migrate error", contractId)
          return
        }
        
      }
    }



    static async new(signerId, envId) {
        const signerKeyStore = new nearAPI.keyStores.UnencryptedFileSystemKeyStore(credentialsPath);
        const signerKeyPair = await signerKeyStore.getKey(nearConfig.networkId, signerId)
        let mainId = "v13-app.bhc8521.testnet"
        if (envId == "testnet") {
          mainId = "app.beepopula.testnet"
        }
        const mainSignerKeyPair = await signerKeyStore.getKey(nearConfig.networkId, mainId)
    
        const keyStore = new nearAPI.keyStores.InMemoryKeyStore()
        const near = await nearAPI.connect({
            keyStore: keyStore,
            ...nearConfig
        });


        return new Contract(mainId, mainSignerKeyPair, signerKeyPair, near)
    }
  
  }

async function sleep(ms) {
  await new Promise((resolve, reject) => {
    setTimeout(() => {
      resolve()
    }, ms)
  })
}

async function upgrade(type, envId, signerId, migrate = false) {
    let contract = await Contract.new(signerId)
    let file = fs.readFileSync(`../res/${type}.wasm`)
    const data = await rp.get(`https://${envId}.popula.io/api/v1/communities/rank?page=0&limit=400&sort=down`)
    const obj = JSON.parse(data)
    for (let community of obj.data) {
        const communityId = community.community_id
        console.log('upgrading...  ' + communityId)
        contract.upgrade(file, communityId, migrate)
        await sleep(3000)
    }
    // await contract.upgrade(file, "lacrovenft.community.beepopula.testnet", migrate)
    // await contract.upgrade(file, "nepbot4near.community.beepopula.testnet", migrate)
    // await contract.upgrade(file, "world4meta1nft2.community-genesis2.bhc8521.testnet", migrate)
    // await contract.upgrade(file, "123.community.beepopula.testnet", migrate)
    // await contract.upgrade(file, "corn-and-cex.community.beepopula.testnet", migrate)
}

async function init() {
  yargs
  .scriptName("multi upgrade")
  .usage('$0 cmd [args]')
  .options({ 
    accountId: { 
      type: 'string',
      describe: 'account ID',
      alias: 'a', 
      hidden: false,
    },
    envId: { 
      type: 'string',
      describe: 'env ID',
      alias: 'e', 
      hidden: false,
    },
    migrate: {
      type: 'bool',
      describe: 'migrate',
      alias: 'm', 
      hidden: false,
    }
  })
  .command('upgrade [type]', 'set a community type', (yargs) => {
    yargs.positional('type', {
      type: 'string',
      default: 'normal',
      describe: 'community type'
    })
  }, async function (argv) {
    upgrade(argv.type, argv.envId, argv.accountId, argv.migrate)
  })
  .argv
}

init()