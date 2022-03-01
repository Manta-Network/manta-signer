// @ts-nocheck
import { SignerInterfaceConfig } from './signerInterfaceConfig';

export class BrowserSignerInterfaceConfig extends SignerInterfaceConfig {
  constructor(bip44CoinTypeId, signerUrl, baseStorageKey) {
    super(bip44CoinTypeId, signerUrl, baseStorageKey);
    this.isInBrowser = true;
    this.baseStorageKey = baseStorageKey;
  }
}
