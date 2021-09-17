int create_account(char *password, char **res, size_t *res_len);
int load_root_seed(char *password, void **res);
int recover_account(unsigned char *root_seed, unsigned char *buffer, const size_t len, void **res, size_t *res_len);
int generate_mint_data(unsigned char *root_seed, unsigned char *buffer, const size_t len, void **res, size_t *res_len);
int batch_generate_private_transfer_data(unsigned char *root_seed, unsigned char *buffer, const size_t len, void **res, size_t *res_len);
int batch_generate_reclaim_data(unsigned char *root_seed, unsigned char *buffer, const size_t len, void **res, size_t *res_len);
int derive_shielded_address(unsigned char *root_seed, unsigned char *buffer, const size_t len, void **res, size_t *res_len);
int generate_asset(unsigned char *root_seed, unsigned char *buffer, const size_t len, void **res, size_t *res_len);
