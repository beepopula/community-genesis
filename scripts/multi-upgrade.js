const rp = require('request-promise')
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

const homedir = require('os').homedir();
const path = require('path');
const CREDENTIALS_DIR = '.near-credentials';
const credentialsPath = path.join(homedir, CREDENTIALS_DIR);

class Contract {

    signerKeyPair
    near
  
    constructor(signerKeyPair, near) {
      this.signerKeyPair = signerKeyPair
      this.near = near
    }

    async upgrade(code, contractId) {
        await this.near.config.keyStore.setKey(nearConfig.networkId, contractId, this.signerKeyPair)
        const account = await this.near.account(contractId)
        await account.deployContract(code)
    }



    static async new(signerId) {
        const signerKeyStore = new nearAPI.keyStores.UnencryptedFileSystemKeyStore(credentialsPath);
        const signerKeyPair = await signerKeyStore.getKey(nearConfig.networkId, signerId)
    
        const keyStore = new nearAPI.keyStores.InMemoryKeyStore()
        const near = await nearAPI.connect({
            keyStore: keyStore,
            ...nearConfig
        });
        //near.config.keyStore.getKey()

        return new Contract(signerKeyPair, near)
    }
  
  }

async function upgrade(type, envId, signerId) {
    let contract = await Contract.new(signerId)
    let file = fs.readFileSync(`../res/${type}.wasm`)
    const data = await rp.get(`https://${envId}.popula.io/api/v1/communities/rank?page=0&limit=100&sort=down`)
    const obj = JSON.parse(data)
    for (let community of obj.data) {
        const communityId = community.community_id
        if (communityId == 'app.beepopula.testnet' || communityId == 'v10-app.bhc8521.testnet') {
            continue
        }
        console.log('upgrading...  ' + communityId)
        await contract.upgrade(file, communityId)
    }
    // await contract.upgrade(file, "nepbotnepbotnepbot.community-genesis2.bhc8521.testnet")
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
  })
  .command('upgrade [type]', 'set a community type', (yargs) => {
    yargs.positional('type', {
      type: 'string',
      default: 'normal',
      describe: 'community type'
    })
  }, async function (argv) {
    upgrade(argv.type, argv.envId, argv.accountId)
  })
  .argv
}

init()