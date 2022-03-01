// @ts-nocheck
import { ClientAsset } from '../..';

export class AssetStore {
  constructor(api) {
    this.api = api;
  }

  reset() {
    this.savePrivateAssets([]);
  }

  savePrivateAssets(privateAssets: ClientAsset[]) {}

  loadPrivateAssets() {}

  addPrivateAssets(newAssets: ClientAsset[]) {
    // Check for duplicates before saving--this avoids a bug:
    // If the user has Manta Web App open in two browser windows, they will both
    // attempt to fetch new assets, and will sometimes save duplicates
    const prevAssets = this.loadPrivateAssets();
    const uniqueNewAssets = newAssets.filter((newAsset) => {
      return !prevAssets.some(prevAsset => prevAsset.equals(newAsset));
    });
    this.savePrivateAssets([...prevAssets, ...uniqueNewAssets]);
  }

  _serializeAssetsForStorage(privateAssets) {
    return privateAssets.map((asset) => Array.from(asset.toU8a(this.api)));
  }

  _deserializeAssetsFromStorage(serializedAssets) {
    const privateAssetsRaw = serializedAssets || [];
    return privateAssetsRaw.map((bytes) =>
      ClientAsset.fromU8a(new Uint8Array(bytes), this.api)
    );
  }
}
