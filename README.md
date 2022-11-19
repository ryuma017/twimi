<div align="center">
<samp>

# Twimi

**A Twitter-like API implemented based on Layered Architecture.**<br>
**(This is a deliverable of the internship at [Yumemi Inc.](https://twitter.com/yumemiinc))**

</samp>
</div>

## How to build (MacOS)

### Pre-requisites

- Docker

```
brew install --cask docker
```

- mold

```
brew install mold
```

- sqlx-cli

```
cargo install sqlx-cli --no-default-features --features rustls,mysql
```

### Build

Create and migrate MySQL database via Docker:

```
./scripts/init_mysql.sh
```

Set a key to encode a JWT with:

```
export SECRET_KEY="your-jwt-secret"
```

Build the server:

```
cargo build
```
