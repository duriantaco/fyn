# Third-party services

## Authentication with alternative package indexes

See the [alternative indexes integration guide](../../guides/integration/alternative-indexes.md) for
details on authentication with popular alternative Python package indexes.

## Hugging Face support

fyn supports automatic authentication for the Hugging Face Hub. Specifically, if the `HF_TOKEN`
environment variable is set, fyn will propagate it to requests to `huggingface.co`.

This is particularly useful for accessing private scripts in Hugging Face Datasets. For example, you
can run the following command to execute the script `main.py` script from a private dataset:

```console
$ HF_TOKEN=hf_... fyn run https://huggingface.co/datasets/<user>/<name>/resolve/<branch>/main.py
```

You can disable automatic Hugging Face authentication by setting the `UV_NO_HF_TOKEN=1` environment
variable.
