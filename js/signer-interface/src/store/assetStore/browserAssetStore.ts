// @ts-nocheck
import { AssetStore } from './assetStore';
import * as store from 'store';
import { ClientAsset } from '../..';

export class BrowserAssetStore extends AssetStore {
  constructor(api, baseStorageKey) {
    super(api);
    this.baseStorageKey = baseStorageKey;
  }

  savePrivateAssets(privateAssets: ClientAsset[]) {
    const serializedAssets = this._serializeAssetsForStorage(privateAssets);
    store.set(`${this.baseStorageKey}PrivateAssets`, serializedAssets);
  }

  loadPrivateAssets(): ClientAsset[] {
    const serializedAssets = store.get(`${this.baseStorageKey}PrivateAssets`);
    return this._deserializeAssetsFromStorage(serializedAssets);
  }
}
