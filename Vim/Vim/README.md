# Vim Cheat Sheet

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

## Movement

- **Arrows** or `h` ←, `j` ↓, `k` ↑, `l` →  
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
- `d<w>` : delete to next word  
- `d$` : delete to end of line  
- `c<w>` : change to next word (enters Insert)  
- `c$` : change to end of line  
- `u` : undo  
- `Ctrl-r` : redo

## Yank & Paste

- `yy` : yank (copy) line  
- `y<w>` : yank to next word  
- `p` : paste after cursor  
- `P` : paste before cursor  
- `10p` : paste 10 times  
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

## Splits & Tabs

- **Horizontal split**  
  - `:split filename` / `:sp filename`  
  - `Ctrl-w s`  
- **Vertical split**  
  - `:vsplit filename` / `:vs filename`  
  - `Ctrl-w v`  
- **Navigate windows**  
  - `Ctrl-w h/j/k/l`  
- **Tabs**  
  - `:tabnew file`  
  - `:tabnext` / `:tabn`  
  - `:tabprev` / `:tabp`  
  - `gt` / `gT`

## Buffers & Files

- `:e filename` : open file  
- `:bnext` / `:bn` : next buffer  
- `:bprev` / `:bp` : previous buffer  
- `:ls` : list buffers  
- `:bd` : delete buffer  

## Marks & Jumps

- `m{a-z}` : set mark  
- `'a` / `` `a `` : jump to mark (line/col)  
- `` '' `` / ``` `` ``` : back to last position  
- `Ctrl-o` / `Ctrl-i` : jump backward/forward in jump list  

## Macros

- `qa` : start recording into register a  
- `q` : stop recording  
- `@a` : play register a  
- `@@` : repeat last macro  

## Indentation & Formatting

- `>>` / `<<` : shift line right/left  
- `=motion` : auto-indent text covered by motion  
- `gg=G` : re-indent entire file  

## Folding

- `zf{motion}` : create fold  
- `zd` : delete fold  
- `zo` / `zc` : open/close fold  
- `za` : toggle  

## Settings (in vimrc or `:`)

- `:set number` / `:set relativenumber`  
- `:set autoindent` / `:set expandtab`  
- `:set shiftwidth=4` / `:set tabstop=4`  
- `:syntax on`  

## Exiting

- `:w` : save  
- `:q` : quit  
- `:wq` / `ZZ` : save & quit  
- `:q!` : quit without saving  

