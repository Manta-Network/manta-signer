// @ts-nocheck
import { AssetStore } from './assetStore';
import * as fs from 'fs';
import { ClientAsset } from '../..';

export class JsonAssetStore extends AssetStore {
  constructor(api, baseFileName) {
    super(api);
    this.fileName = `${baseFileName}AssetStore.json`;
    if (!fs.existsSync(this.fileName)) {
      const emptyAssetStore = JSON.stringify([]);
      fs.writeFileSync(this.fileName, emptyAssetStore);
    }
  }

  savePrivateAssets(privateAssets: ClientAsset[]) {
    const serializedAssets = JSON.stringify(
      this._serializeAssetsForStorage(privateAssets)
    );
    fs.writeFileSync(this.fileName, serializedAssets);
  }

  loadPrivateAssets() {
    const serializedAssets = JSON.parse(fs.readFileSync(this.fileName));
    return this._deserializeAssetsFromStorage(serializedAssets);
  }
}
