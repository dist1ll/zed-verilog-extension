# Zed Verilog Extension

Adds Verilog and SystemVerilog support for [Zed](https://zed.dev/) using
[slang-server](https://github.com/hudson-trading/slang-server).

## Features

- Treesitter based syntax highlighting
- Diagnostics
- Auto-completion
- Hover documentation
- Formatting (format-on-save is disabled by default)

## Installation

1. Clone [this fork](https://github.com/dist1ll/zed-verilog-extension).
2. Open Zed and run `zed: install dev extension` from the command palette.
3. Select the cloned extension directory.
4. Open a Verilog or SystemVerilog file.

## Language Server

This fork supports only slang-server. It uses `slang-server` from your PATH when
available, otherwise it downloads the pinned release from Hudson River Trading.
To explicitly select it in Zed's settings:

```jsonc
"languages": {
  "SystemVerilog": {
    "language_servers": ["slang"]
  }
}
```

## Configuration

Configure editor behavior under `languages.SystemVerilog` in Zed's settings.
For slang-server's project configuration, see the
[slang-server documentation](https://hudson-trading.github.io/slang-server/start/config/).

To use a specific local binary and bypass automatic downloads, set its absolute
path under `lsp.slang.binary.path` in Zed's settings.

## License

This project is licensed under the [Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0) license or the [MIT](http://opensource.org/licenses/MIT) license, at your option. See [COPYRIGHT](./COPYRIGHT) for more details.
