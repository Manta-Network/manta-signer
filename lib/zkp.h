int generate_transfer(char *app_version, char *buffer, const size_t len, char **res, size_t *res_len);
int generate_reclaim(char *app_version, char *buffer, const size_t len, char **res, size_t *res_len);
int derive_shielded_address(char *path, int asset_id, char **res, size_t *res_len);
int generate_asset(int asset_id, char *value, char *path, char **res, size_t *res_len);