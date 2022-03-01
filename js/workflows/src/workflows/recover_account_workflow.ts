import { ApiPromise } from '@polkadot/api';
import {
  SignerInterface,
  NodeJsSignerInterfaceConfig,
  ClientAsset
} from 'signer-interface';

export type ConfigRecoverAccountWorkflow = {
  signerInterface: NodeJsSignerInterfaceConfig;
};

export async function recoverAccountWorkflow(
  config: ConfigRecoverAccountWorkflow,
  api: ApiPromise
): Promise<Array<ClientAsset>> {
  const signerInterface = new SignerInterface(api, config.signerInterface);
  const recoveredAccount = await signerInterface.recoverAccount();
  return recoveredAccount;
}
