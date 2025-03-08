1. Make sure `rustc` version `>1.85.0` is installed
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Build and run the project
    ```bash
    cargo run
    ```

3. Send a request to your webserver
    ```bash
    curl 127.0.0.1:6188 -v
    ```