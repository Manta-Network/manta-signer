// @ts-nocheck
import { AddressStore } from './addressStore';
import * as fs from 'fs';

export class JsonAddressStore extends AddressStore {
  fileName: string;

  constructor(bip44CoinTypeId, baseFileName) {
    super(bip44CoinTypeId);
    this.fileName = `${baseFileName}AddressStore.json`;
    if (!fs.existsSync(this.fileName)) {
      const emptyAddressStore = JSON.stringify({
        internalAddresses: [],
        externalAddresses: [],
        uncommitedOffset: 0
      });
      fs.writeFileSync(this.fileName, emptyAddressStore);
    }
  }

  _loadInternalAddresses() {
    return JSON.parse(fs.readFileSync(this.fileName)).internalAddresses;
  }

  _loadExternalAddresses() {
    return JSON.parse(fs.readFileSync(this.fileName)).externalAddresses;
  }

  _loadUncommitedOffset() {
    return JSON.parse(fs.readFileSync(this.fileName)).uncommitedOffset;
  }

  _saveInternalAddresses(internalAddresses) {
    const storeJson = JSON.parse(fs.readFileSync(this.fileName));
    storeJson.internalAddresses = internalAddresses;
    fs.writeFileSync(this.fileName, JSON.stringify(storeJson));
  }

  _saveExternalAddresses(externalAddresses) {
    const storeJson = JSON.parse(fs.readFileSync(this.fileName));
    storeJson.externalAddresses = externalAddresses;
    fs.writeFileSync(this.fileName, JSON.stringify(storeJson));
  }

  _saveUncommitedOffset(uncommitedOffset) {
    const storeJson = JSON.parse(fs.readFileSync(this.fileName));
    storeJson.uncommitedOffset = uncommitedOffset;
    fs.writeFileSync(this.fileName, JSON.stringify(storeJson));
  }
}
