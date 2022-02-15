// @ts-nocheck
import {
  BIP_44_PURPOSE_INDEX,
  DEFAULT_ACCOUNT_ID,
  EXTERNAL_CHAIN_ID,
  INTERNAL_CHAIN_ID
} from '../../constants/bip44Constants';

export class AddressStore {
  constructor(coinTypeId) {
    this.baseKeypath = `m/${BIP_44_PURPOSE_INDEX}'/${coinTypeId}'/${DEFAULT_ACCOUNT_ID}'`;
  }

  reset() {
    this._saveInternalAddresses([]);
    this._saveExternalAddresses([]);
    this._saveUncommitedOffset(0);
  }

  _loadInternalAddresses() {}

  _loadExternalAddresses() {}

  _loadUncommitedOffset() {}

  _saveInternalAddresses(addresses) {}

  _saveExternalAddresses(addresses) {}

  _saveUncommitedOffset(uncommitedOffset) {}

  _incrementUncommitedOffset() {
    this._saveUncommitedOffset(this._loadUncommitedOffset() + 1);
  }

  _resetUncommitedOffset() {
    this._saveUncommitedOffset(0);
  }

  saveInternalAddress(address) {
    const internalAddresses = this._loadInternalAddresses();
    internalAddresses.push(address);
    this._saveInternalAddresses(internalAddresses);
    this._incrementUncommitedOffset();
  }

  saveExternalAddress(address) {
    const externalAddresses = this._loadExternalAddresses();
    externalAddresses.push(address);
    this._saveExternalAddresses(externalAddresses);
  }

  rollBackInternalAddresses() {
    const internalAddresses = this._loadInternalAddresses();
    for (let i = 0; i < this._loadUncommitedOffset(); i++) {
      internalAddresses.pop();
    }
    this._saveInternalAddresses(internalAddresses);
    this._resetUncommitedOffset();
  }

  commitInternalAddresses() {
    this._resetUncommitedOffset();
  }

  getNextInternalKeypath() {
    const addressIdx = this._loadInternalAddresses().length;
    return `${this.baseKeypath}/${INTERNAL_CHAIN_ID}/${addressIdx}`;
  }

  getCurrentInternalKeypath() {
    const addressIdx = this._loadInternalAddresses().length - 1;
    return `${this.baseKeypath}/${INTERNAL_CHAIN_ID}/${addressIdx}`;
  }

  getNextExternalKeypath() {
    const addressIdx = this._loadExternalAddresses().length;
    return `${this.baseKeypath}/${EXTERNAL_CHAIN_ID}/${addressIdx}`;
  }
}
