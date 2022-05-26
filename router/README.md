# Router

## Deployment

### Environment Variables

| Variable      | Description                                                               | Default                    |
| ------------- | ------------------------------------------------------------------------- | -------------------------- |
| `DB_URI`      | The sqlite db connection URI.                                             | `sqlite://./db.db`         |
| `GAIA_ADDR`   | The address at which gaia is accessible at.                               | `http://gaia-backend:8081` |
| `JWT_PEM_LOC` | The location to the PEM file that contains the JWT signing key.           | `/certs/jwt-key.pem`       |
| `HMAC_KEY`    | A random key (string) used to generate HMAC signatures for dynamic flags. | ``                         |
