// @ts-nocheck
import { base64Encode } from '@polkadot/util-crypto';

export class BlockchainNodeClient {
  constructor(api) {
    this.api = api;
  }

  // Returns all on-chain void numbers
  async getAllVoidNumbers() {
    const entries = await this.api.query.mantaPay.voidNumbers.entries();
    const voidNumbers = entries
      .map((storageItem) => storageItem[0].args)
      .flat();
    return voidNumbers;
  }

  // Returns all UTXOS and encrypted notes void numbers that we haven't yet
  // attempted to recover, along with an updted set of UTXOs we have attempted
  // to recover. Attempting to recover a UTXO-encrypted note pair with Manta Signer
  // is computationally expensive, so we don't want to perform this operation redundantly
  async getNewUTXOsAndEncryptedNotes(utxoSet) {
    const newUTXOs = [];
    const newEncryptedNotes = [];
    const entries = await this.api.query.mantaPay.ledgerShards.entries();
    const pairs = entries.map((entry) => entry[1]);

    pairs.forEach(([utxo, encryptedNote]) => {
      const utxoStr = base64Encode(utxo.slice(0, 8));
      if (!utxoSet[utxoStr]) {
        newUTXOs.push(utxo);
        newEncryptedNotes.push(encryptedNote);
        utxoSet[utxoStr] = true;
      }
    });

    return { newUTXOs, newEncryptedNotes, utxoSet };
  }
}
