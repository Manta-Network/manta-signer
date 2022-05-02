import { ApiPromise, Keyring } from '@polkadot/api';
import { SignerInterface, NodeJsSignerInterfaceConfig } from 'signer-interface';
import { makeTxResHandler } from '../utils/MakeTxResHandler';
import TxStatus from '../utils/TxStatus';
import { selectCoins } from 'coin-selection';
import * as BN from 'bn.js';
import { Wallet, Transaction } from 'manta-wasm-wallet';
import { base58Decode, base58Encode } from '@polkadot/util-crypto';

export type ConfigPrivateTransferAssetWorkflow = {
  assetId: number;
  valueAtomicUnits: number;
  receiver: string;
  polkadotJsSigner: string;
  signerInterface: NodeJsSignerInterfaceConfig;
};



export async function privateTransferAssetWorkflow(
  config: ConfigPrivateTransferAssetWorkflow,
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

    const bytes = base58Decode(config.receiver);
    const addressJson = JSON.stringify({
        spend: Array.from(bytes.slice(0, 32)),
        view: Array.from(bytes.slice(32)),
    });
    const value = config.valueAtomicUnits.toString();
    const assetId = config.assetId;
    const txJson = `{ "PrivateTransfer": [{ "id": ${assetId}, "value": "${value}" }, ${addressJson} ]}`;
    const transaction = Transaction.from_string(txJson);
    wallet.wasmApi.setTxResHandler(txResHandler)
    return (async () => {
        return await wallet.wallet.post(transaction, null);
    })();
  });

  return await status;
}
