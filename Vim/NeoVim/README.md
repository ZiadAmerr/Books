# NeoVim Cheat Sheet

## Modes

- **Normal** (Esc) – navigate, edit, issue commands  
- **Insert** – insert text  
  - `i` : before cursor  
  - `I` : beginning of line  
  - `a` : after cursor  
  - `A` : end of line  
  - `o` : new line below  
  - `O` : new line above  
- **Visual** – select text  
  - `v` : character-wise  
  - `V` : line-wise  
  - `Ctrl-v` : block-wise  
- **Command-line** – Ex commands  
  - `:` then command (e.g. `:w`, `:q`, `:e file`)
- **Terminal** – embedded terminal  
  - `:terminal` or `:term` to open
  - `Ctrl-\ Ctrl-n` to exit to Normal mode

## NeoVim-Specific Features

### Built-in Terminal

- `:terminal` or `:term` – open terminal in current window
- `:split | terminal` – open terminal in horizontal split
- `:vsplit | terminal` – open terminal in vertical split
- `Ctrl-\ Ctrl-n` – exit terminal mode to normal mode

### LSP (Language Server Protocol)

- `:LspInfo` – show LSP status
- `gd` – go to definition
- `K` – show hover information
- `gr` – show references
- `<leader>rn` – rename symbol (with proper config)
- `<leader>ca` – code actions (with proper config)

### Tree-sitter Integration

- `:TSInstall [language]` – install language parser
- `:TSModuleInfo` – show tree-sitter module info
- Enhanced syntax highlighting and code navigation

### Lua Configuration

- `init.lua` instead of `init.vim`
- `:lua print("Hello")` – run Lua code
- `:luafile file.lua` – run Lua file

## Movement

- **Arrows** or `h` $\leftarrow$, `j` $\downarrow$, `k` $\uparrow$, `l` $\rightarrow$  
- **Words**  
  - `w` : next word start  
  - `e` : end of word  
  - `b` : previous word start  
- **Lines**  
  - `0` : start  
  - `^` : first non-blank  
  - `$` : end  
  - `gg` : top  
  - `G` : bottom  
  - `5j` / `5k` : 5 lines down/up  
- **Search**  
  - `/pattern` → next  
  - `?pattern` → previous  
  - `n` / `N` : repeat

## Editing & Deletion

- `x` : delete char  
- `dd` : delete line  
- `d<motion>` : delete to motion  
- `c<motion>` : change to motion (enters Insert)  
- `u` : undo  
- `Ctrl-r` : redo

## Yank & Paste

- `yy` : yank (copy) line  
- `y<motion>` : yank to motion  
- `p` : paste after cursor  
- `P` : paste before cursor  
- `"+y` / `"+p` : use system clipboard

## Visual Mode Operations

- Select with `v`/`V`/`Ctrl-v`, then:
  - `y` : yank  
  - `d` : delete  
  - `c` : change (enter Insert)  
  - `>` / `<` : indent/unindent  

## Search & Replace

- `:%s/old/new/g` : replace all in file  
- `:s/old/new/g` : replace in current line  
- `:%s/old/new/gc` : confirm each  

## Splits & Windows

- **Horizontal split**  
  - `:split filename` / `:sp filename`  
  - `Ctrl-w s`  
- **Vertical split**  
  - `:vsplit filename` / `:vs filename`  
  - `Ctrl-w v`  
- **Navigate windows**  
  - `Ctrl-w h/j/k/l`
  - `Ctrl-w w` : cycle through windows

## Tabs

- `:tabnew file` : open file in new tab
- `:tabnext` / `:tabn` : next tab
- `:tabprev` / `:tabp` : previous tab
- `gt` / `gT` : next/previous tab

## Buffers & Files

- `:e filename` : open file  
- `:bnext` / `:bn` : next buffer  
- `:bprev` / `:bp` : previous buffer  
- `:ls` : list buffers  
- `:bd` : delete buffer  

## Plugin Management
NeoVim has built-in support for plugins:

### Built-in Package Management

- Add plugins to `~/.local/share/nvim/site/pack/*/start/` for auto-loading
- Add plugins to `~/.local/share/nvim/site/pack/*/opt/` for optional loading
- `:packadd plugin-name` to load optional plugin

### Popular Plugin Managers

- **packer.nvim** (Lua-based)
- **vim-plug**
- **dein.vim**

## Configuration

- Default config location: `~/.config/nvim/`
- `init.lua` or `init.vim` for main configuration
- Lua modules in `~/.config/nvim/lua/`

## Useful NeoVim Settings

```lua
-- In init.lua
vim.opt.number = true
vim.opt.relativenumber = true
vim.opt.mouse = 'a'
vim.opt.termguicolors = true
vim.opt.clipboard = 'unnamedplus'
vim.opt.inccommand = 'split'  -- live preview of substitutions
```

## Exiting

- `:w` : save  
- `:q` : quit  
- `:wq` / `ZZ` : save & quit  
- `:q!` : quit without saving

## Unique NeoVim Commands

- `:checkhealth` – diagnostics tool to check NeoVim setup
- `:messages` – view message history
- `:lua` – execute Lua code
- `:help nvim-features` – NeoVim-specific features documentation
