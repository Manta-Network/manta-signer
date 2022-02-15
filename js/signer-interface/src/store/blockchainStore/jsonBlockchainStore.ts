// @ts-nocheck
import { BlockchainStore } from './blockchainStore';
import * as fs from 'fs';

export class JsonBlockchainStore extends BlockchainStore {
  constructor(baseFileName) {
    super();
    this.fileName = `${baseFileName}BlockchainStore.json`;
    if (!fs.existsSync(this.fileName)) {
      const emptyAddressStore = JSON.stringify({});
      fs.writeFileSync(this.fileName, emptyAddressStore);
    }
  }

  loadUTXOSet() {
    return JSON.parse(fs.readFileSync(this.fileName));
  }

  saveUTXOSet(utxoSet) {
    fs.writeFileSync(this.fileName, JSON.stringify(utxoSet));
  }
}
