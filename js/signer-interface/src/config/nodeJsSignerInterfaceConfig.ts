// @ts-nocheck
import { SignerInterfaceConfig } from './signerInterfaceConfig';

export class NodeJsSignerInterfaceConfig extends SignerInterfaceConfig {
  constructor(bip44CoinTypeId, signerUrl, baseStorageKey) {
    super(bip44CoinTypeId, signerUrl, baseStorageKey);
    this.isInBrowser = false;
    this.baseStorageKey = baseStorageKey;
  }
}
