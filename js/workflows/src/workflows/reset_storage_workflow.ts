import { ApiPromise } from '@polkadot/api';
import { NodeJsSignerInterfaceConfig, SignerInterface } from 'signer-interface';

export type ConfigResetStorageWorkflow = {
  signerInterface: NodeJsSignerInterfaceConfig;
};

export function resetStorageWorkflow(
  config: ConfigResetStorageWorkflow,
  api: ApiPromise
) {
  const signerInterface = new SignerInterface(api, config.signerInterface);
  signerInterface.resetStorage();
}
