# raw import

## Settings

`.config/raw-import/configuration.toml`

Example:

```toml
[input]
path = './'
filter = ['*.cr3', '*.raf']

[output]
path = 'hello/{yyyy}/{yyyy-MM-dd}/'
filename = '{yyyy-MM-dd} {HH:mm:ss} {seq} {filename}.{extension}'
```
