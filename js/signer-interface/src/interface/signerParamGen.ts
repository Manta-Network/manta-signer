// @ts-nocheck
import { base58Decode } from '@polkadot/util-crypto';

export class SignerParamGen {
  constructor(api) {
    this.api = api;
  }

  generateAssetParams(assetId, keypath, value) {
    return this.api.createType('GenerateAssetParams', {
      asset_id: assetId,
      keypath: keypath,
      value: value
    });
  }

  generateMintParams(asset) {
    const res = this.api.createType('GenerateAssetParams', {
      asset_id: asset.assetId,
      keypath: asset.keypath,
      value: asset.valueAtomicUnits
    });
    return res;
  }

  generateAddressParams(keypath) {
    return this.api.createType('DeriveShieldedAddressParams', {
      keypath: keypath
    });
  }

  generatePrivateTransferParams(
    inputAsset1,
    inputAsset2,
    changeOutputAsset,
    nonChangeOutputAsset
  ) {
    const params = this.api.createType('GeneratePrivateTransferParams', {
      sender_asset_1_value: inputAsset1.valueAtomicUnits,
      sender_asset_2_value: inputAsset2.valueAtomicUnits,
      sender_asset_1_keypath: inputAsset1.keypath,
      sender_asset_2_keypath: inputAsset2.keypath,
      sender_asset_1_shard: inputAsset1.shard,
      sender_asset_2_shard: inputAsset2.shard,
      change_output_keypath: changeOutputAsset.keypath,
      non_change_output_keypath: nonChangeOutputAsset.keypath,
      non_change_output_value: nonChangeOutputAsset.valueAtomicUnits,
      change_output_value: changeOutputAsset.valueAtomicUnits
    });
    return params;
  }

  generateReclaimParams(
    reclaimInputAsset1,
    reclaimInputAsset2,
    changeOutputAsset
  ) {
    const totalValueAtomicUnits = reclaimInputAsset1.valueAtomicUnits.add(
      reclaimInputAsset2.valueAtomicUnits
    );
    const reclaimValueAtomicUnits = totalValueAtomicUnits.sub(
      changeOutputAsset.valueAtomicUnits
    );
    const params = this.api.createType('GenerateReclaimParams', {
      asset_id: reclaimInputAsset1.assetId,
      input_asset_1_value: reclaimInputAsset1.valueAtomicUnits,
      input_asset_2_value: reclaimInputAsset2.valueAtomicUnits,
      input_asset_1_keypath: reclaimInputAsset1.keypath,
      input_asset_2_keypath: reclaimInputAsset2.keypath,
      input_asset_1_shard: reclaimInputAsset1.shard,
      input_asset_2_shard: reclaimInputAsset2.shard,
      change_keypath: changeOutputAsset.keypath,
      reclaim_value: reclaimValueAtomicUnits
    });
    return params;
  }

  generatePrivateTransferBatchParams(
    assetId,
    receivingAddress,
    privateTransferParamsList
  ) {
    return this.api.createType('GeneratePrivateTransferBatchParams', {
      asset_id: assetId,
      receiving_address: base58Decode(receivingAddress),
      private_transfer_params_list: privateTransferParamsList
    });
  }

  generateReclaimBatchParams(privateTransferParamsList, reclaimParams) {
    return this.api.createType('GenerateReclaimBatchParams', {
      private_transfer_params_list: privateTransferParamsList,
      reclaim_params: reclaimParams
    });
  }

  generateRecoverAccountParams(utxos, encryptedNotes, voidNumbers) {
    return this.api.createType('RecoverAccountParams', {
      void_numbers: voidNumbers,
      utxos: utxos,
      encrypted_notes: encryptedNotes
    });
  }
}
