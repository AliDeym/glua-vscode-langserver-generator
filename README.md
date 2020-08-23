# glua-vscode-langserver-generator

Generates Intellisense documentations for Garry's mod in [VSCode's lua language server](https://github.com/sumneko/vscode-lua).

  Currently supports:
  

 - [x] Libraries
 - [x] Globals
 - [x] Objects
 - [ ] Panels
 - [ ] Enums

## Steps to Produce
You may build the [Lua Language Server](https://marketplace.visualstudio.com/items?itemName=sumneko.lua) without installing it, and include the generated language files into your Language Server. However, there is no difference since the language server is not compiled, rather bundled with the extension.

1. Install [Lua Language Server](https://marketplace.visualstudio.com/items?itemName=sumneko.lua).
2. Install [GMod Wiki Scrapper](https://www.npmjs.com/package/gmod-wiki-scraper).
3. Run `gmod-wiki-scrapper` in your command-line and wait for the process to finish downloading files.
4. Rename `output` directory to `input`, and move it to your cloned directory (Same level as `Cargo.toml`).
5. Run the program: 
<br />`cargo run`
6. Copy and replace the generated folders inside `data` folder, into language server:
<br />`{vscode user directory}/extensions/sumneko.lua-0.xx.x/server/`
7. Start Visual Studio, and try all the GLua docs in your intellisense.
