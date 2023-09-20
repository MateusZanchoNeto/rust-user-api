# Rust CRUD User API
    
This is a simple CRUD API made in Rust to handling a User data.
The main goal of this project is to make it simple to understand how to receive a HTTP request in Rust, handle it and give correct responses.

### License

This project is licensed under the [**MIT License**](LICENSE)

### Building

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. [Install Docker](https://docs.docker.com/engine/install/)
3. Clone this repository
```
git clone https://github.com/MateusZanchoNeto/rust-user-api.git
```
4. Navigate to the project directory
```
cd rust-user-api
```
5. Get the database up in a container named db
```
docker compose up -d db
```
6. Build the app in another container named rustapp
```
docker compose build
```
7. Get the API up in the rustapp container
```
docker compose up rustapp
```

# Notes
To access the database
```
docker exec -it db psql -U postgres
```

# References

- [Rust 🦀 CRUD Rest API - Rust, Postgres, Docker, Docker Compose](https://youtu.be/vhNoiBOuW94)
- [Rust Actix-Web Api - Full Crud](https://youtu.be/TY0BLKCVMiU)
