- Make sure `rustc` version `>1.85.0` is installed -> `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`


- Build and run the project -> `cargo run`


- Run 2 server:
   - Create virtualenv -> `python3.12 -m venv .venv`
   - Activate it -> `./venv/bin/activate`
   - Install requirements -> `pip install -r requirements.txt`
   - Run first server -> `panther run --port 8000 --reload`
   - Run first server -> `panther run --port 8001 --reload`
  

- Send a request to your webserver -> `curl 127.0.0.1:6188 -v`
