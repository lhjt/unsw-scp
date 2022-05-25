# Gaia Backend

## Deployment

### Environment Variables

| Variable      | Description                                                    | Default                         |
| ------------- | -------------------------------------------------------------- | ------------------------------- |
| `JWT_PEM_LOC` | Location to the pem that contains the JWT key.                 | `../../proxy/certs/jwt-key.pem` |
| `DB_URI`      | The sqlite db connection URI.                                  | `sqlite://./db.db`              |
| `PASETO_KEY`  | 32 byte string for signing paseto download tokens.             | ``                              |
| `PUBLIC_ADDR` | The public address from which this service is accessible from. | `login.local.host:8443`         |
