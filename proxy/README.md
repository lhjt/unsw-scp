# Main Proxy

This component is responsible for all the network routing requests and is the main endpoint that users will reach when communicating with the CTF platform. It exhibits the use of mTLS to identify clients.

## Deployment

### Environment Variables

| Name           | Description                                                            | Default      |
| -------------- | ---------------------------------------------------------------------- | ------------ |
| `BASE_DOMAIN`  | The public-facing base domain on which the proxy will be reachable at. | `local.host` |
| `REGISTRY_URL` | The URL where the service registry provider is available at.           | `registry`   |
