// @ts-nocheck
import { AddressStore } from './addressStore';
import * as store from 'store';

export class BrowserAddressStore extends AddressStore {
  constructor(bip44CoinTypeId, baseStorageKey) {
    super(bip44CoinTypeId);
    this.externalAddressesStorageKey = `${baseStorageKey}ExternalAddresses`;
    this.internalAddressesStorageKey = `${baseStorageKey}InternalAddresses`;
    this.internalAddressesUncommitedOffset = `${this.baseStorageKey}InternalAddressesUncommitedOffset`;
  }

  _loadInternalAddresses() {
    return store.get(this.internalAddressesStorageKey, []);
  }

  _loadExternalAddresses() {
    return store.get(this.externalAddressesStorageKey, []);
  }

  _loadUncommitedOffset() {
    return store.get(this.internalAddressesUncommitedOffset, 0);
  }

  _saveInternalAddresses(addresses) {
    return store.set(this.internalAddressesStorageKey, addresses);
  }

  _saveExternalAddresses(addresses) {
    return store.set(this.externalAddressesStorageKey, addresses);
  }

  _saveUncommitedOffset(offset) {
    store.set(this.internalAddressesUncommitedOffset, offset);
  }
}
