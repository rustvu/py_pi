# py_pi

Implementing Monte-Carlo-based π estimation as a Python module.

## Requirements

This example uses [uv](https://docs.astral.sh/uv/) for python project and tooling.
Please make sure you have it installed. See the [Installing uv](https://docs.astral.sh/uv/getting-started/installation/) page.

Obviously, you also need Rust installed. See the [Installing Rust](https://www.rust-lang.org/tools/install) page.

## Running the example

1. Install `uv` (see above) and clone this repository.

2. Build the project (in editable mode) by running:

   ```bash
   uvx maturin develop
   ```

3. Open the `experiment.ipynb` Jupyter Notebook by running:

   ```bash
   uv run jupyter notebook experiment.ipynb
   ```

## Tests

To run the tests, use:

```bash
uv run --with ".[tests]" pytest
```
