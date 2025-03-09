
## Our WebServer
<b>It is the best.</b>

---

### Installation
- <details>
    <summary>1. Pull its image</summary>
    <pre>docker pull ...</pre>

  </details>

- <details>
    <summary>2. Write your config file in a Toml format</summary>
    <pre>Check out the ...</pre>

  </details>

- <details open>
    <summary>3. Run ...</summary>
      <pre>I'll tell you later</pre>

  </details>

---

### Be a Contributer

- #### Install `rust`
    Make sure `rustc` version `>1.85.0` is installed
    ```shell
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

- #### Build and Run

    ```shell
    cargo run
    ```

- #### Run 2 servers
    - Create virtualenv
        ```shell
        python3.12 -m venv .venv
        ```
    - Activate it
        ```shell
        ./venv/bin/activate
        ```
    - Create virtualenv
        ```shell
        pip install -r requirements.txt
        ```
    - Create virtualenv
        ```shell
        panther run --port 8000 --reload
        ```
    - Create virtualenv
        ```shell
        panther run --port 8001 --reload
        ```

- #### Send a Request to your WebServer

    ```shell
    curl 127.0.0.1:6188 -v
    ```

- #### Improve Documentation
  
    ```shell
    pip install -r requirements.txt
    ```

    ```shell
    cd docs/
    ```

    ```shell
    mkdocs serve
    ```
