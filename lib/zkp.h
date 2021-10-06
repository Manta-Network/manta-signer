// Persistence
int create_account(char *password, char **res);
int save_recovered_account(char *password, char *recovery_phrase);
int load_root_seed(char *password, void **res);

// Sensitive payload gen
int batch_generate_private_transfer_data(unsigned char *root_seed, unsigned char *buffer, const size_t len, void **res, size_t *res_len);
int batch_generate_reclaim_data(unsigned char *root_seed, unsigned char *buffer, const size_t len, void **res, size_t *res_len);

// Non-sensitive payload gen
int recover_account(unsigned char *root_seed, unsigned char *buffer, const size_t len, void **res, size_t *res_len);
int generate_mint_data(unsigned char *root_seed, unsigned char *buffer, const size_t len, void **res, size_t *res_len);
int derive_shielded_address(unsigned char *root_seed, unsigned char *buffer, const size_t len, void **res, size_t *res_len);
int generate_asset(unsigned char *root_seed, unsigned char *buffer, const size_t len, void **res, size_t *res_len);

// Transaction batch summary info for Signer UI
  // Private transfers
int get_private_transfer_batch_params_value(unsigned char *buffer, const size_t len, char **res);
int get_private_transfer_batch_params_currency_symbol(unsigned char *buffer, const size_t len, char **res);
int get_private_transfer_batch_params_recipient(unsigned char *buffer, const size_t len, char **res);;
  // Reclaims
int get_reclaim_batch_params_value(unsigned char *buffer, const size_t len, char **res);
int get_reclaim_batch_params_currency_symbol(unsigned char *buffer, const size_t len, char **res);
