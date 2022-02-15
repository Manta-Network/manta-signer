import { ApiPromise, Keyring } from '@polkadot/api';
import { SignerInterface, NodeJsSignerInterfaceConfig } from 'signer-interface';
import { makeTxResHandler } from '../utils/MakeTxResHandler';
import TxStatus from '../utils/TxStatus';
import { selectCoins } from 'coin-selection';
import * as BN from 'bn.js';

export type ConfigReclaimAssetWorkflow = {
  assetId: number;
  valueAtomicUnits: number;
  polkadotJsSigner: string;
  signerInterface: NodeJsSignerInterfaceConfig;
};

export async function reclaimAssetWorkflow(
  config: ConfigReclaimAssetWorkflow,
  api: ApiPromise
): Promise<TxStatus> {
  const keyring = new Keyring({ type: 'sr25519' });
  const polkadotJsSigner = keyring.addFromUri(config.polkadotJsSigner);
  const signerInterface = new SignerInterface(api, config.signerInterface);

  const assets = await signerInterface.recoverAccount();
  const coinSelection = selectCoins(
    new BN(config.valueAtomicUnits),
    assets,
    config.assetId
  );
  const reclaimTxs = await signerInterface.buildReclaimTxs(coinSelection);
  const status: Promise<TxStatus> = new Promise((resolve) => {
    const txResHandler = makeTxResHandler(
      api,
      (block) => {
        console.log(TxStatus.finalized(block));
        signerInterface.cleanupTxSuccess();
        resolve(TxStatus.finalized(block));
      },
      (block, error) => {
        signerInterface.cleanupTxFailure();
        console.log(TxStatus.failed(block, error));
        resolve(TxStatus.failed(block, error));
      }
    );
    api.tx.utility
      .batch(reclaimTxs)
      .signAndSend(polkadotJsSigner, txResHandler);
  });

  return await status;
}
