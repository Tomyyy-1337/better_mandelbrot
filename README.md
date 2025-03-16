## Run benchmark
* Benchmark f64
    ```sh
    cargo bench repaint
    ```
* Benchmark fpdec::Decimal
    ```sh
    cargo bench repaint -F quad
    ```
* Benchmark f256
    ```sh
    cargo bench repaint -F octo
    ```