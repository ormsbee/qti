# QTI Library for Rust/WebAssembly

**Experiment in making a QTI lib that could be created in Rust but used in JavaScript and Python.**

There is almost nothing here at the moment. I plan to pursue this as a side
hobby to learn more about both Rust and QTI.

### 🛠️ Browser: Build with `wasm-pack`

```
wasm-pack build --target web
```

Then run a simple webserver in the root `qti` directory, such as:

```
python3 -m http.server 8000
```

Then open [http://localhost:8000/test.html](http://localhost:8000/test.html) in
your browser.

### 🛠️ Python: Build with `maturin`

You need to install `matruin`, a Python library. From the root `qti` directory,
do the following:

```
python -m venv .env
source .env/bin/activate
pip install maturin
matruin develop
```

Then run the following script to see it work:

```
python basic.py
```
