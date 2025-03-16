## Run benchmark
* Run double precision only
    ```sh
    cargo run --example bench --release
    ```
* Run quadruple precision only
    ```sh
    cargo run --example bench --release -F quad
    ```
* Run octuple precision only
    ```sh
    cargo run --example bench --release -F octo
    ```
* Run all tests
    ```sh
    cargo run --example bench --release -F full
    ```
