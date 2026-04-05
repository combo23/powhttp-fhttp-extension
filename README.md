# fhttp Headers

A [powhttp](https://powhttp.com) extension that copies request headers to your clipboard in Go's [fhttp](https://github.com/bogdanfinn/fhttp) library format.

## Usage

Right-click any request in powhttp and select **"Copy to fhttp"**. The headers will be copied to your clipboard in a ready-to-paste format:

```go
req.Header = http.Header{
    "content-type":        {"application/json"},
    "accept":              {"*/*"},
    "sec-ch-ua":           {"\"Chromium\";v=\"146\", \"Google Chrome\";v=\"146\""},
    "user-agent":          {"Mozilla/5.0 ..."},
    "accept-encoding":     {"gzip, deflate, br, zstd"},
    "cookie":              {"session=abc123"},
    http.HeaderOrderKey:   {"content-type", "accept", "sec-ch-ua", "user-agent", "accept-encoding", "cookie"},
    http.PHeaderOrderKey:  {":method", ":authority", ":scheme", ":path"},
}
```

### Features

- Pseudo headers (`:method`, `:authority`, etc.) are separated into `http.PHeaderOrderKey`
- Duplicate header names (e.g. multiple `cookie` entries) appear only once in `http.HeaderOrderKey`
- Quotes in header values are properly escaped
- Header columns are aligned for readability

## Building

```sh
cargo build --release
```

The binary will be at `./target/release/fhttp_headers`.

## Installation

Point powhttp to this extension directory. The `extension.json` manifest configures the extension to run the release binary.
