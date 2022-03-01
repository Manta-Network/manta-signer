import { ApiPromise, Keyring } from '@polkadot/api';
import { makeTxResHandler } from '../utils/MakeTxResHandler';
import TxStatus from '../utils/TxStatus';
import { SignerInterface, NodeJsSignerInterfaceConfig } from 'signer-interface';

export type ConfigMintAssetWorkflow = {
  assetId: number;
  valueAtomicUnits: number;
  polkadotJsSigner: string;
  signerInterface: NodeJsSignerInterfaceConfig;
};

export async function mintAssetWorkflow(
  config: ConfigMintAssetWorkflow,
  api: ApiPromise
): Promise<TxStatus> {
  const keyring = new Keyring({ type: 'sr25519' });
  const signer = keyring.addFromUri(config.polkadotJsSigner);
  const signerInterface = new SignerInterface(api, config.signerInterface);

  const mintTx = await signerInterface.buildMintTx(
    config.assetId,
    config.valueAtomicUnits
  );

  const status: Promise<TxStatus> = new Promise((resolve) => {
    const txResHandler = makeTxResHandler(
      api,
      (block) => {
        signerInterface.cleanupTxSuccess();
        console.log(TxStatus.finalized(block));
        resolve(TxStatus.finalized(block));
      },
      (block, error) => {
        signerInterface.cleanupTxFailure();
        console.log(TxStatus.failed(block, error));
        resolve(TxStatus.failed(block, error));
      }
    );

    mintTx.signAndSend(signer, txResHandler);
  });

  return await status;
}
