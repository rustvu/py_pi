# py_pi

Implementing Monte-Carlo-based π estimation as a Python module.

## Requirements

It is highly recommended to build and run this code in a Python virtual environment.
The easiest way is to use the [Anaconda distribution](https://www.anaconda.com/), but you can use your preinstalled Python interpreter, also.

### Option A: Anaconda

1. Install [Anaconda](https://www.anaconda.com/)

2. In the command prompt (Anaconca Prompt on Windows), create a new virtual environment (with some key packages installed):

```sh
conda create -n py_pi python jupyter numpy maturin
```

3. Activate your environment

```sh
conda activate py_pi
```

4. Build (release profile) and install this package in your virtual environment

```
maturin develop -r
```

5. Launch Jupyter Lab, and open the `experiment.ipynb` notebook in your browser. Play with it.

```sh
jupyter lab
```

### Option B: Pre-installed Python

Needs at least Python 3.7.

1. In the command prompt, create a new virtual environment:

```sh
python3 -m venv .venv
```

2. Activate your environment

    - On macOS/Linux:

    ```sh
    source .venv/bin/activate
    ```

    - On Windows:

    ```sh
    .\Scripts\activate.bat
    ```

3. Install required Python packages

```sh
pip install jupyter numpy maturin
```

4. Build (release profile) and install this package in your virtual environment

```
maturin develop -r
```

5. Launch Jupyter Lab, and open the `experiment.ipynb` notebook in your browser. Play with it.

```sh
jupyter lab
```