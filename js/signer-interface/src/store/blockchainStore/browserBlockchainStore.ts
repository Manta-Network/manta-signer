// @ts-nocheck
import * as store from 'store';
import { BlockchainStore } from './blockchainStore';

export class BrowserBlockchainStore extends BlockchainStore {
  constructor(baseStorageKey) {
    super();
    this.utxoSetStorageKey = `${baseStorageKey}UTXOSet`;
  }

  loadUTXOSet() {
    return store.get(this.utxoSetStorageKey, {});
  }

  saveUTXOSet(utxoSet) {
    store.set(this.utxoSetStorageKey, utxoSet);
  }
}
