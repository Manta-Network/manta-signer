// @ts-nocheck
export class SimulatedLedgerState {
  constructor(api) {
    this.api = api;
    this.offChainAssets = [];
  }

  addOffChainAsset(offChainAsset) {
    this.offChainAssets.push(offChainAsset);
  }

  async getShard(asset) {
    const shardOnChain = await this._getShardOnChain(asset.shardIndex);
    return this._addOffChainAssetsToShard(shardOnChain, asset.shardIndex);
  }

  async _getShardOnChain(shardIndex) {
    const storage = await this.api.query.mantaPay.ledgerShards.entries(
      shardIndex
    );
    return storage.map((storageItem) => storageItem[1][0]);
  }

  _addOffChainAssetsToShard(shardOnChain, shardIndex) {
    const shardOffChain = this.offChainAssets
      .filter((offChainAsset) => offChainAsset.shardIndex === shardIndex)
      .map((asset) => asset.utxo);
    return [...shardOnChain, ...shardOffChain];
  }
}
