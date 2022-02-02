import { ApiPromise } from '@polkadot/api';
import { SignerInterface, NodeJsSignerInterfaceConfig } from 'signer-interface';

export type ConfigDeriveAddressWorkflow = {
  signerInterface: NodeJsSignerInterfaceConfig;
};

export async function deriveAddressWorkflow(
  config: ConfigDeriveAddressWorkflow,
  api: ApiPromise
): Promise<string> {
  const signerInterface = new SignerInterface(api, config.signerInterface);
  const address = await signerInterface.generateNextExternalAddress();
  return address;
}
