# Server Configuration

## Default Configuration

Use these environment variables for configuration in docker

```env
HOST="localhost"
PORT="80"
VERBOSE="4"
SECRET="a093c76bd2c5f4e7dff6360c78bcb57a" # random
# LOG_PATH="server.log"

POSTGRES__USER="admin"
POSTGRES__PASSWORD="secret"
POSTGRES__HOST="0.0.0.0"
POSTGRES__DBNAME="postgres"

HTTPS__PORT="8443"
HTTPS__KEY="./key.pem"
HTTPS__CERT="./cert.pem"
HTTPS__REDIRECT="true"

MAIL__FROM="noreply@example.com"
MAIL__USERNAME="admin"
MAIL__PASSWORD="secret"
MAIL__HOST="0.0.0.0"
```