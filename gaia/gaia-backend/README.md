# Gaia Backend

## Deployment

### Environment Variables

| Variable      | Description                                    | Default                         |
| ------------- | ---------------------------------------------- | ------------------------------- |
| `JWT_PEM_LOC` | Location to the pem that contains the JWT key. | `../../proxy/certs/jwt-key.pem` |
| `DB_URI`      | The sqlite db connection URI                   | `sqlite://./db.db`              |
