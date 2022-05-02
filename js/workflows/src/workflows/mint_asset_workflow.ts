import { ApiPromise, Keyring } from '@polkadot/api';
import { makeTxResHandler } from '../utils/MakeTxResHandler';
import TxStatus from '../utils/TxStatus';
import { SignerInterface, NodeJsSignerInterfaceConfig } from 'signer-interface';
import { Wallet, Transaction } from 'manta-wasm-wallet';

export type ConfigMintAssetWorkflow = {
  assetId: number;
  valueAtomicUnits: number;
  polkadotJsSigner: string;
  signerInterface: NodeJsSignerInterfaceConfig;
};

export async function mintAssetWorkflow(
  config: ConfigMintAssetWorkflow,
  api, 
  wallet
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
    const txJson = `{ "Mint": { "id": ${assetId}, "value": "${value}" }}`;
    const transaction = Transaction.from_string(txJson);
    wallet.wasmApi.setTxResHandler(txResHandler)
    return (async () => {
        return await wallet.wallet.post(transaction, null);
    })();
  });

  return await status;
}
