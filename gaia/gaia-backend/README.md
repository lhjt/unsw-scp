# Gaia Backend

## Deployment

### Environment Variables

| Variable        | Description                                                    | Default                         |
| --------------- | -------------------------------------------------------------- | ------------------------------- |
| `JWT_PEM_LOC`   | Location to the pem that contains the JWT key.                 | `../../proxy/certs/jwt-key.pem` |
| `DB_URI`        | The sqlite db connection URI.                                  | `sqlite://./db.db`              |
| `PASETO_KEY`    | 32 byte string for signing paseto download tokens.             | ``                              |
| `PUBLIC_ADDR`   | The public address from which this service is accessible from. | `login.local.host:8443`         |
| `FROM_ADDR`     | The email address from which emails are sent to clients.       | `noreply@local.host`            |
| `SMTP_ADDR`     | The address of the SMTP server to use to send emails.          | ``                              |
| `SMTP_USERNAME` | The username to use with the SMTP server.                      | ``                              |
| `SMTP_PASSWORD` | The password to use with the SMTP server.                      | ``                              |
