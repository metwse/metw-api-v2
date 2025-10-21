# metw-api-v2
metw.cc API version 2 source tree.

## Contributing
<img width="96" height="96" alt="no-ai-slop" align="right" src="https://github.com/user-attachments/assets/bca16d5a-a6fe-4cbf-b41f-1176e000cff2" />

Contributions are welcome! Please check our
[Code of Conduct](http://github.com/metwse/code-of-conduct) before submitting
pull requests.

AI-generated code is strongly discouraged.

## API Documenatation
The project exposes an OpenAPI specification at:
```
https://localhost:1186/openapi.json
```
This endpoint provides a complete machine-readable API schema that describes
all available endpoints, request/response types, and authentication details.

You can view and interact with the API documentation using Swagger UI or any
OpenAPI-compatible client.

## Environment Setup
Create a `.env` file (you can copy `.env.example`):
```sh
DATABASE_URL=postgres://metwcc:db_password@localhost:2345/metw-v2
# Change db password from metw-db which is in the Dockerfile and put it here to db_password part 
REDIS_URL=redis://localhost/
# Put your redis server url to redis_host part
JWT_SECRET=CHANGEME
# Change JWT Secret to something secure
HOST=127.0.0.1:1186
# Change your host and port if it is different from default
```
> These environment variables are used by the backend for connecting to
  PostgreSQL and Redis.

If the `HOST` environment variable is not set, the application will bind to the
local loopback address `127.0.0.1` on port `1186` by default.

## Development Environment
You can start a local PostgreSQL instance using the provided Dockerfile inside
the `db/` folder.

### Build the Image
```sh
cd db
docker build -t metw-db .
```

### Run the Container
This setup maps the container’s internal PostgreSQL port `5432` to port `2345`
on your host machine.

You can change the host port (`2345`) if needed -- just make sure to update the
`DATABASE_URL` accordingly. The internal container port (`5432`) should
remain unchanged.
```sh
docker run -d \
  --name metw-db \
  -p 2345:5432 \
  metw-db
```

This starts PostgreSQL with the initial schema defined in SQL files in `db/`.

### Testing with Development Database
For testing, use the `.env.test` file and a temporary database instance. By
default, the database Docker image does not include fixtures. If you want to
add mock data (from `db/fixtures`), you can enable it at build time by passing
the `--build-arg fixtures=yes` flag.

To start the test database:
```sh
cd db
docker build --build-arg fixtures=yes -t metw-db-test .
docker run -d -p 2345:5432 --name metw-db-test metw-db-test
```
> **Note**: You need to enable fixtures to run tests.

Once the test database is up, you can run your tests:
```sh
cargo test
```

### Cleanup
These commands stop and remove both the development and test database
containers. If you only want to remove one of them, simply pass its name as an
argument.
```sh
docker stop metw-db metw-db-test
docker rm --force metw-db metw-db-test
```
