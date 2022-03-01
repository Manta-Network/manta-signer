// @ts-nocheck
import BN from 'bn.js';

export class ClientAsset {
  assetId: number;
  valueAtomicUnits: BN;
  keypath: string;
  utxo: Uint8Array;
  voidNumber: Uint8Array;
  shardIndex: number;
  shard: Array<Uint8Array>;

  constructor(
    assetId,
    valueAtomicUnits,
    keypath = null,
    utxo = null,
    voidNumber = null,
    shardIndex = null
  ) {
    this.assetId = assetId;
    this.valueAtomicUnits = valueAtomicUnits;
    this.keypath = keypath;
    this.utxo = utxo;
    this.voidNumber = voidNumber;
    this.shardIndex = shardIndex;
    this.shard = null;
  }

  static fromSignerAsset(signerAsset) {
    return new ClientAsset(
      signerAsset.asset_id.toNumber(),
      signerAsset.value,
      signerAsset.keypath,
      signerAsset.utxo,
      signerAsset.void_number,
      signerAsset.shard_index
    );
  }

  static fromU8a(u8a, api) {
    const signerAsset = api.createType('MantaSignerInputAsset', u8a);
    return ClientAsset.fromSignerAsset(signerAsset);
  }

  static newExternalOutput(assetId, valueAtomicUnits) {
    return new ClientAsset(assetId, valueAtomicUnits);
  }

  toSignerAsset(api) {
    return api.createType('MantaSignerInputAsset', {
      asset_id: this.assetId,
      value: this.valueAtomicUnits,
      utxo: this.utxo,
      keypath: this.keypath,
      void_number: this.voidNumber,
      shard_index: this.shardIndex
    });
  }

  toU8a(api) {
    return this.toSignerAsset(api).toU8a();
  }

  setShard(shard) {
    this.shard = shard;
  }

  equals(other) {
    return this.utxo.join(',') === other.utxo.join(',');
  }
}
