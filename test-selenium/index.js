const {waitReady} = require('@polkadot/wasm-crypto');
const {ApiPromise, Keyring, WsProvider} = require('@polkadot/api');
const {CryptoPerson, PRIVATE_ASSETS, PUBLIC_ASSETS} = require('./user');
const {getFailedExtrinsicError} = require('./util');
const sim = require('./out.json');
const {BN} = require('@polkadot/util');

const BOB = '//Bob';
const CHARLIE = '//Charlie';
const DAVE = '//Dave';

const BOB_PORT = 3000;
const CHARLIE_PORT = 3001;
const DAVE_PORT = 3002;

const WS_URL = 'ws://localhost:9800';

const SYMBOL_TO_ASSET_ID = {
    DOL: 1,
    KAR: 8,
    aUSD: 9,
    KSM: 10,
    ROC: 11,
    kBTC: 12,
    MOVR: 13,
};

const DECIMALS = {
    DOL: 18,
    KAR: 12,
    aUSD: 12,
    KSM: 12,
    ROC: 12,
    kBTC: 8,
    MOVR: 18,
};

const ASSET_LOCATIONS = {
    KAR: {
          V1: {
            parents: 1,
            interior: {
              X2: [
                {
                  Parachain: 2000
                },
                {
                  GeneralKey: "0x0080"
                }
              ]
            }
          }
    },
    aUSD: {
          V1: {
            parents: 1,
            interior: {
              X2: [
                {
                  Parachain: 2000
                },
                {
                  GeneralKey: "0x0081"
                }
              ]
            }
          }
    },
    KSM: {
          V1: {
            parents: 1,
            interior: {
              X2: [
                {
                  Parachain: 2000
                },
                {
                  GeneralKey: "0x0083"
                }
              ]
            }
          }
    },
    ROC: {
          V1: {
            parents: 1,
            interior: "Here"
          }
    },
    kBTC: {
          V1: {
            parents: 1,
            interior: {
              X1: {
                Parachain: 2099
              }
            }
          }
    },
    MOVR: {
          V1: {
            parents: 1,
            interior: {
              X1: {
                Parachain: 2214
              }
            }
          }
    }
};


const sleep = (seconds) => { return new Promise(r => setTimeout(r, seconds * 1000)); };


async function register_asset(api, alice, symbol) {
    const decimals = DECIMALS[symbol];
    const loc = ASSET_LOCATIONS[symbol];

    await new Promise((resolve) => {
        let resultHandler = (result) => {
            if (result.status.isFinalized) {
                const msg = getFailedExtrinsicError(result.events, api);
              if (msg != null) {
                  throw Error(`transfer failed: ${msg}`);
              } else {
                  const id = result.status.asFinalized.toHex();
                  console.log(`${symbol} registration complete: ${id}`);
              }
              resolve();
          } else if (result.status.isInBlock) {
                console.log(`INBLOCK`);
          } else {
                console.log(`Something else happened: ${result.status}`);
          }
        };

        let meta = {
            name: symbol,
            symbol: symbol,
            decimals: decimals,
            is_frozen: false,
            min_balance: 1,
            is_sufficient: true 
        };
      
        api.tx.sudo.sudo(
            api.tx.assetManager.registerAsset(loc, meta)
        ).signAndSend(alice, resultHandler);
    });
}


(async () => {
    await waitReady();
    const provider = new WsProvider(WS_URL);
    const api = await ApiPromise.create({ provider });
    const keyring = new Keyring({type: 'sr25519'});
    const alice = keyring.addFromUri('//Alice');
    await api.isReady;

    const bob = await CryptoPerson.initialize(BOB, BOB_PORT);
    const charlie = await CryptoPerson.initialize(CHARLIE, CHARLIE_PORT);
    const dave = await CryptoPerson.initialize(DAVE, DAVE_PORT);
    const accounts = [bob, charlie, dave];

    for (const [symbol, config] of Object.entries(SYMBOL_TO_ASSET_ID)) {
        if (symbol == "DOL") {
            continue;
        }
        await register_asset(api, alice, symbol);
    }
    console.log("Assets registered... pre-funding accounts now");
          
    for (const [index, initial] of sim.initial_accounts.entries()) {
      await init_account(api, alice, accounts, index, initial);
    };

    for (const {type, source_index, asset, sender_index, receiver_index} of sim.updates.filter(i => i.asset.id == 0)) {
        console.log(`Transaction: ${type}`);
        if (type == 'Mint') {
            console.log(`Mint Source ${source_index} asset ${asset}`);

            const acct = accounts[source_index];
            await acct.public_private_send(acct.private_account, PUBLIC_ASSETS[asset.id], `${asset.value}`, "Success");
        } else if (type == 'Reclaim') {
            console.log(`Reclaim asset ${asset} sender_index ${sender_index}`);

            const acct = accounts[sender_index];
            await acct.private_public_send(acct.public_account, PRIVATE_ASSETS[asset.id], `${asset.value}`, "Success");
        } else if (type == 'PrivateTransfer') {
            console.log(`PvtTrans asset ${asset} sender_index ${sender_index} receiver_index ${receiver_index}`);

            const from = accounts[sender_index];
            const to = accounts[receiver_index];

            await from.private_private_send(to.private_account, PRIVATE_ASSETS[asset.id], `${asset.value}`, "Success");
        } else if (type == 'PublicDeposit') {
            console.log(`PubDeps Source index ${source_index} asset ${asset}`);
        } else if (type == 'PublicWithdraw') {
            console.log(`PubWth Source index ${source_index} asset ${asset}`);
        } else {
             throw Error(`Unhandled tx type! ${type}`);
        }
        console.log("Transaction complete");
    }

    for (const [index, final_balance] of sim.final_accounts.entries()) {
        const acct = accounts[index];
        console.log(`Balances for: ${acct.path}`);

        // Public.
        for (const [id, amount] of Object.entries(final_balance.public)) {
            const symbol = PUBLIC_ASSETS[id];
            const public_balance = await acct.public_balance(symbol);
            console.log(`${symbol}: should have ${amount}, has ${public_balance}`);
        }

        // Private.
        for (const [id, amount] of Object.entries(final_balance.secret)) {
            const symbol = PRIVATE_ASSETS[id];
            const private_balance = await acct.private_balance(symbol);
            console.log(`${symbol}: should have ${amount}, has ${private_balance}`);
        }
    }
})()


async function init_account(api, alice, accounts, index, initial) {
    let to_acct = accounts[index];
    for (const [assetIdx, value] of Object.entries(initial.public)) {
        let idx = parseInt(assetIdx);
        let symbol = PUBLIC_ASSETS[idx];
        let assetId = SYMBOL_TO_ASSET_ID[symbol];

        try {
            let resultHandler = (result) => {
                if (result.status.isFinalized) {
                    const msg = getFailedExtrinsicError(result.events, api);
        		    if (msg != null) {
        		        throw Error(`transfer failed: ${msg}`);
        		    } else {
        		        const id = result.status.asFinalized.toHex();
        		        console.log(`${symbol} transfer complete: ${assetId}, hash: ${id}`);
        		    }
        		    unsub();
        	    } else if (result.status.isInBlock) {
                    console.log(`INBLOCK`);
        	    } else {
                    console.log(`Something else happened: ${result.status}`);
        	    }
            };

            let nonce = await api.rpc.system.accountNextIndex(alice.address);
            console.log(`${to_acct.public_account} receiving ${value} ${symbol} (${assetId})`);
            const unsub = await api.tx.sudo.sudoUncheckedWeight(
                api.tx.assetManager.mintAsset(parseInt(assetId), to_acct.public_account, new BN(value)), 1
            ).signAndSend(alice, {nonce}, resultHandler);
        } catch (e) {
            throw Error(`Somethin' messed up happened ${e}`);
        }
        await sleep(1);
    }
}

