# Vault interaction order

1. Service read `role_id` and `secret_id` to auth in vault;
	For local tests use port forward from Lens `kubectl port-forward pods/vault-0 8200:8200 -n vault`
2. Service read `key` and `iv` from vault;
2. Service read encrypted private keys from db;
3. Service decrypt private keys using `key` and `iv` from vault.