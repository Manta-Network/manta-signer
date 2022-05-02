import {Api} from 'manta-wasm-wallet-api';
import { ApiPromise, WsProvider } from '@polkadot/api'
import { waitReady } from '@polkadot/wasm-crypto'

export async function initDolphinApi (url: string): Promise<ApiPromise > {
  await waitReady()
  const wsProvider = new WsProvider(url)
  return ApiPromise.create({ provider: wsProvider })
}


export async function getUserWallet(url, polkadotJsApi, polkadotJsSigner) {
    const {
        Wallet,
        Signer,
        PolkadotJsLedger,
        Transaction,
        Asset,
        AssetId,
        ReceivingKeyRequest
    } = await import('manta-wasm-wallet');

    const wasmApi = new Api(polkadotJsApi, polkadotJsSigner);
    const signer = new Signer(url);
    const ledger = new PolkadotJsLedger(wasmApi);
    let wallet = new Wallet(ledger, signer);
    wallet.sync();
    const receivingKeys = await wallet.receiving_keys(new ReceivingKeyRequest('GetAll'));

    wasmApi.setTxResHandler(({ status, events }) => {
        if (status.isFinalized) {
            for (const event of events) {
                if (polkadotJsApi.events.utility.BatchInterrupted.is(event.event)) {
                    console.log('TODO: failure')
                    // handle failed tx here
                } else if (polkadotJsApi.events.utility.BatchCompleted.is(event.event)) {
                    console.log('TODO: success')
                    // handle successful tx here
                }
            }
        }
    });

    return { wasmApi, signer, ledger, wallet, receivingKeys };
}
