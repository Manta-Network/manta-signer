export const DolphinTypes = {
  CurrencyId: {
    _enum: [
      "DOL"
    ]
  },
  Address: 'MultiAddress',
  LookupSource: 'MultiAddress',
  AssetBalance: 'u128',
  AssetId: 'u32',
  MantaRandomValue: '[u8; 32]',
  MantaSecretKey: '[u8; 32]',
  MantaPublicKey: '[u8; 32]',
  UTXO: '[u8; 32]',
  MantaEciesCiphertext: {
    encrypted_msg: '[u8; 36]',
    ephemeral_pk: '[u8; 32]'
  },
  MantaAssetShieldedAddress: {
    k: 'MantaRandomValue',
    s: 'MantaRandomValue',
    ecpk: 'MantaSecretKey'
  },
  SenderData: {
    void_number: 'MantaRandomValue',
    root: 'MantaRandomValue',
    shard_index: 'u8'
  },
  ReceiverData: {
    cm: 'MantaRandomValue',
    encrypted_note: 'MantaEciesCiphertext'
  },
  MintData: {
    asset_id: 'AssetId',
    value: 'AssetBalance',
    cm: 'MantaRandomValue',
    k: 'MantaRandomValue',
    s: 'MantaRandomValue',
    encrypted_note: 'MantaEciesCiphertext'
  },
  PrivateTransferData: {
    sender_0: 'SenderData',
    sender_1: 'SenderData',
    receiver_0: 'ReceiverData',
    receiver_1: 'ReceiverData',
    proof: '[u8; 192]'
  },
  ReclaimData: {
    asset_id: 'AssetId',
    sender_0: 'SenderData',
    sender_1: 'SenderData',
    receiver: 'ReceiverData',
    reclaim_value: 'AssetBalance',
    proof: '[u8; 192]'
  },
  DeriveShieldedAddressParams: {
    keypath: 'String'
  },
  GenerateAssetParams: {
    asset_id: 'AssetId',
    value: 'AssetBalance',
    keypath: 'String'
  },
  GeneratePrivateTransferParams: {
    sender_asset_1_value: 'AssetBalance',
    sender_asset_2_value: 'AssetBalance',
    sender_asset_1_keypath: 'String',
    sender_asset_2_keypath: 'String',
    sender_asset_1_shard: 'Vec<[u8; 32]>',
    sender_asset_2_shard: 'Vec<[u8; 32]>',
    change_output_keypath: 'String',
    non_change_output_keypath: 'Option<String>',
    non_change_output_value: 'AssetBalance',
    change_output_value: 'AssetBalance'
  },
  GeneratePrivateTransferBatchParams: {
    asset_id: 'AssetId',
    receiving_address: 'MantaAssetShieldedAddress',
    private_transfer_params_list: 'Vec<GeneratePrivateTransferParams>'
  },
  PrivateTransferBatch: {
    private_transfer_data_list: 'Vec<PrivateTransferData>'
  },
  GenerateReclaimParams: {
    asset_id: 'AssetId',
    input_asset_1_value: 'AssetBalance',
    input_asset_2_value: 'AssetBalance',
    input_asset_1_keypath: 'String',
    input_asset_2_keypath: 'String',
    input_asset_1_shard: 'Vec<[u8; 32]>',
    input_asset_2_shard: 'Vec<[u8; 32]>',
    change_keypath: 'String',
    reclaim_value: 'AssetBalance'
  },
  GenerateReclaimBatchParams: {
    private_transfer_params_list: 'Vec<GeneratePrivateTransferParams>',
    reclaim_params: 'GenerateReclaimParams'
  },
  ReclaimBatch: {
    private_transfer_data_list: 'Vec<PrivateTransferData>',
    reclaim_data: 'ReclaimData'
  },
  RecoverAccountParams: {
    void_numbers: 'Vec<MantaRandomValue>',
    utxos: 'Vec<UTXO>',
    encrypted_notes: 'Vec<MantaEciesCiphertext>'
  },
  MantaSignerInputAsset: {
    keypath: 'String',
    asset_id: 'AssetId',
    value: 'AssetBalance',
    utxo: 'UTXO',
    shard_index: 'u8',
    void_number: 'MantaRandomValue'
  },
  RecoveredAccount: {
    assets: 'Vec<MantaSignerInputAsset>'
  },
  ShardMetaData: {
    current_index: 'u64',
    current_auth_path: '[u8; 584]'
  }
};
