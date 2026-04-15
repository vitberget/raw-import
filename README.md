# RAW import

From your terminal, import your raw files from ...whatever... to configured ...place.

## Settings

`.config/raw-import/configuration.toml`

Example:

```toml
[input]
path = './'
filter = ['*.cr2', '*.cr3', '*.raf']
recursive = true

[output]
path = 'hello/{yyyy}/{yyyy}-{MM}-{dd}/'
filename = "{yyyy}-{MM}-{dd} {HH}:{mm}:{ss} {seq} {filename}.{extension}"
```

## Help

```bash
raw-import --help

```

