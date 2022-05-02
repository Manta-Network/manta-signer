import { ApiPromise, Keyring } from '@polkadot/api';
import { makeTxResHandler } from '../utils/MakeTxResHandler';
import TxStatus from '../utils/TxStatus';

export type ConfigTransferAssetWorkflow = {
  assetId: number;
  valueAtomicUnits: number;
  polkadotJsSigner: string;
  receiver: string;
};

export async function transferAssetWorkflow(
  config: ConfigTransferAssetWorkflow,
  api: ApiPromise
): Promise<TxStatus> {
  const keyring = new Keyring({ type: 'sr25519' });
  const signer = keyring.addFromUri(config.polkadotJsSigner);

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
    api.tx.mantaPay
      .publicTransfer({id: config.assetId, value: config.valueAtomicUnits}, config.receiver)
      // @ts-ignore FIXME
      .signAndSend(signer, txResHandler);
  });
  return await status;
}
