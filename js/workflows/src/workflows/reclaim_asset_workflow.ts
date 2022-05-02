import { ApiPromise, Keyring } from '@polkadot/api';
import { SignerInterface, NodeJsSignerInterfaceConfig } from 'signer-interface';
import { makeTxResHandler } from '../utils/MakeTxResHandler';
import TxStatus from '../utils/TxStatus';
import { selectCoins } from 'coin-selection';
import * as BN from 'bn.js';
import { Wallet, Transaction } from 'manta-wasm-wallet';



export type ConfigReclaimAssetWorkflow = {
  assetId: number;
  valueAtomicUnits: number;
  polkadotJsSigner: string;
  signerInterface: NodeJsSignerInterfaceConfig;
};

export async function reclaimAssetWorkflow(
  config: ConfigReclaimAssetWorkflow,
  api: ApiPromise,
  wallet,
): Promise<TxStatus> {
  const status: Promise<TxStatus> = new Promise((resolve) => {
    const txResHandler = makeTxResHandler(
      api,
      (block) => {
        console.log(TxStatus.finalized(block));
        resolve(TxStatus.finalized(block));
      },
      (block, error) => {
        console.log(TxStatus.failed(block, error));
        resolve(TxStatus.failed(block, error));
      }
    );

    const value = config.valueAtomicUnits.toString();
    const assetId = config.assetId;
    const txJson = `{ "Reclaim": { "id": ${assetId}, "value": "${value}" }}`;
    const transaction = Transaction.from_string(txJson);
    wallet.wasmApi.setTxResHandler(txResHandler)

    return (async () => {
        return await wallet.wallet.post(transaction, null);
    })();
  });

  return await status;
}
