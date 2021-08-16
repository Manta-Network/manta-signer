int generate_transfer(char *app_version, char *buffer, const size_t len, char **res, size_t *res_len);
int generate_reclaim(char *app_version, char *buffer, const size_t len, char **res, size_t *res_len);
int derive_shielded_address(char *path, int asset_id, char **res, size_t *res_len);
int generate_asset(int asset_id, char *value, char *path, char **res, size_t *res_len);
char *generate_recovery_phrase(const char *password);
int modify_password_by_recovery_phrase(const char *recovery_phrase, const char *password);
int verify_password(const char *password);
// 是否创建账户,0为未创建，1为已创建
int account_created();