// @ts-nocheck
import * as axios from 'axios';
import { base58Encode } from '@polkadot/util-crypto';
import { ClientAsset } from '../classes/clientAsset';
export class SignerClient {
  constructor(api, signerURL) {
    axios.defaults.baseURL = signerURL;
    this.api = api;
  }

  async getSignerVersion() {
    try {
      const res = await axios.get('version', { timeout: 500 });
      return res.data.version;
    } catch (timeoutError) {
      return null;
    }
  }

  async recoverAccount(params) {
    const res = await axios.post('recoverAccount', params.toU8a());
    const account = this.api.createType(
      'RecoveredAccount',
      new Uint8Array(res.data.recovered_account)
    );
    return account.assets.map((signerAsset) =>
      ClientAsset.fromSignerAsset(signerAsset)
    );
  }

  async deriveShieldedAddress(params) {
    const res = await axios.post('deriveShieldedAddress', params.toU8a());
    return base58Encode(
      this.api
        .createType(
          'MantaAssetShieldedAddress',
          new Uint8Array(res.data.address)
        )
        .toU8a()
    );
  }

  async generateAsset(params) {
    const res = await axios.post('generateAsset', params.toU8a());
    return this.api.createType(
      'MantaSignerInputAsset',
      new Uint8Array(res.data.asset)
    );
  }

  async generateMintData(params) {
    const res = await axios.post('generateMintData', params.toU8a());
    return new Uint8Array(res.data.mint_data);
  }

  async requestGeneratePrivateTransferData(params) {
    const res = await axios.post('generatePrivateTransferData', params.toU8a());
    const decoded = this.api.createType(
      'PrivateTransferBatch',
      new Uint8Array(res.data.private_transfer_data)
    );
    return decoded.private_transfer_data_list.map((data) => data.toU8a());
  }

  async requestGenerateReclaimData(params) {
    const res = await axios.post('generateReclaimData', params.toU8a());
    return this.api.createType(
      'ReclaimBatch',
      new Uint8Array(res.data.reclaim_data)
    );
  }
}
