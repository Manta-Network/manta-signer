int recover_account(unsigned char *buffer, const size_t len, void **res, size_t *res_len);
int generate_mint_data(unsigned char *buffer, const size_t len, void **res, size_t *res_len);
int generate_private_transfer_data(unsigned char *buffer, const size_t len, void **res, size_t *res_len);
int generate_reclaim_data(unsigned char *buffer, const size_t len, void **res, size_t *res_len);
int derive_shielded_address(unsigned char *buffer, const size_t len, void **res, size_t *res_len);
int generate_asset(unsigned char *buffer, const size_t len, void **res, size_t *res_len);
