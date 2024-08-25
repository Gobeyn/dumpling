# Dumpling

![Dumpling TUI](./examples/example.jpg)

Dumpling is a light-weight, minimal research paper manager from the terminal. With the CLI tool, basic information on research papers 
can be created, and subsequently be explored by the TUI. Inside the TUI a minimal amount of essential actions can be performed. 

By default, the application assumes papers are stored in `$HOME/.paper/`. The paper information files created by the CLI tool are stored in the 
`$HOME/.cache/dumpling` directory which is always generated if it does not exists when the program is executed. If there is any weird behaviour, 
check out the `dumpling.log` file which is also stored in the `$HOME/.cache/dumpling/` directory. The program assumes PDF files are located 
in the `$HOME/.paper/` directory, but this can be changed with the configuration file, which the program will look for in `$HOME/.config/dumpling/`.

For reference, we list what the above used directory expand to in different operating systems for as user named `USER`. For more alias conversions 
see the [`dirs` crate](https://docs.rs/crate/dirs/latest).

| Alias | Linux | MacOS | Windows |
|-------|-------|-------|---------|
| `$HOME/` | `/home/USER` | `Users/USER/` | `C:\Users\USER\` |
| `$HOME/.cache/` | `/home/USER/.cache/` | `/Users/USER/Library/Caches/` | `C:\Users\USER\AppData\Local\` |
| `$HOME/.config/` | `/home/USER/.config/` | `/Users/USER/Library/Application Support/` | `C:\Users\USER\AppData\Roaming\` | 

The `~` alias is also supported, so any instance of `$HOME` can be replaced with `~` if preferred.

## Installation

For easy installation, a `Makefile` is provided. It is assumed that [`GNU Make`](https://www.gnu.org/software/make/) and 
the [Rust ecosystem (e.g. `rustup` and `cargo`)](https://www.rust-lang.org/tools/install) are installed on you system. To build the application run:
```bash
make
```
To copy the resulting compiled application to the /usr/bin/ directory so it can be accessed globally in you system run:
```bash
make install
```
Copying the file to that directory is not required, you could also `export` the path to the binary in your `.bashrc`, `.zshrc`, etc.
Lastly, to clean files from the compilation process:
```bash 
make clean
```

## Usage 

Dumpling provides a single CLI tool called `dumpling`. The following 
arguments can be attached to it:

| Short form | Long form | Argument | Action |
|------------|-----------|----------|--------|
| -t | --title | "\[TITLE\]" | Set title of paper |
| -y | --year | INT | Set year of publication |
| -j | --journal | "\[JOURNAL\]" | Set journal the paper was published in |
| / | --desc | "\[DESCRIPTION\]" | Short description of the papers contents |
| -b | --bibtex | "\[BIB\]" | Bibtex formatted reference for the paper |
| / | --doc | "\[DOCNAME\]" | Name of the PDF document, the directory is set in the configuration file and should not be provided |
| / | --filter-tag | "\[TAG\]" |Show only papers with certain tag. This only does something if the TUI is opened.|
| -a | --author | "\[AUTHOR\]" | Add author for the paper, this option can be used multiple times. |
| / | --tag | "\[TAG\]" | Add tag to paper, this option can be used multiple times. |
| -o | --open | No argument | Open the TUI. |
| / | --list-tags | No argument | List all the tags used and how often they appear. |
| / | --pdf-diagnose | No argument | Show all the PDF file paths mentioned in the paper files that are invalid, i.e. the file it points to does not exists. Also show all the PDF files in the `pdf_dir` that are not mentioned by any paper file | 
| / | --auto | No argument | If `--bibtex` is provided, the contents of it are used to automatically infer `--title`, `--year`, `--journal` and all the `--authors`. The result can be overwritten by using those flags anyway. | 
| -h | --help | No argument | Print the help menu. |
| / | --version | No argument | Print package information |

Here is an example usage:
```bash
dumpling -t "The Casimir Energy with Perfect Electromagnetic Boundary Conditions and Duality: a Field Theoretic Approach" -y 2024 -j "Preprint arXiv" --desc "Computes the Casimir energy for PEMC boundary conditions between two parallel plates using the electromagnetic field tensor and path integrals" -b "@article{dudal2024casimir,
  title={The Casimir energy with perfect electromagnetic boundary conditions and duality: a field-theoretic approach},
  author={Dudal, David and Gobeyn, Aaron and Oosthuyse, Thomas and Stouten, Sebbe and Vercauteren, David},
  journal={arXiv preprint arXiv:2406.19743},
  year={2024}
}" --doc "Dudal_2024.pdf" -a "David Dudal" -a "Aaron Gobeyn" -a "Thomas Oosthuyse" -a "Sebbe Stouten" -a "David Vercauteren" --tag "Casimir" --tag "PEMC" --tag "EM tensor" --tag "Path integral"
```
Running it will create a new paper information file, stored in `$HOME/.paper` directory. The file name will not be recognizable because it is created by SHA256 encoding of the file contents. 
The same result can be achieved utilising the `--auto` flag,
```bash
dumpling --desc "Computes the Casimir energy for PEMC boundary conditions between two parallel plates using the electromagnetic field tensor and path integrals" -b "@article{dudal2024casimir,
  title={The Casimir energy with perfect electromagnetic boundary conditions and duality: a field-theoretic approach},
  author={Dudal, David and Gobeyn, Aaron and Oosthuyse, Thomas and Stouten, Sebbe and Vercauteren, David},
  journal={arXiv preprint arXiv:2406.19743},
  year={2024}
}" --doc "Dudal_2024.pdf" --tag "Casimir" --tag "PEMC" --tag "EM tensor" --tag "Path integral" --auto
```
The title, year and authors of the paper are inferred from the given bibtex citation. If we then run:
```bash
dumpling -o
```
We will see this paper as inside the TUI. By pressing `b` the bibtex citation for this paper will be put into the system clipboard, and can be pasted where needed. Suppose we made made a second entry: 
```bash
dumpling -t "Title" --tag "Tag"
```
If we open the TUI as before, both papers will appear, but if instead we run:
```bash
dumpling -o --filter-tag "Tag"
```
Only the paper titled Test will show up. We can see all the tags being used by running: 
```bash
dumpling --list-tags
```
In the first entry we made during this example, we set `--doc` to "Dudal_2024.pdf". We can see the status of this file by running: 
```bash
dumpling --pdf-diagnose
```
This will tell us if there are any PDF files mentioned by the paper information files that are not present in `$HOME/.paper/`, and if there are any files in that directory that are not mentioned by a paper information file.

## Configuration

The user can create their own configuration for certain elements of the program with a configuration file. When starting the program, it will search 
for the presence of `$HOME/.config/dumpling/dumpling.toml`, in case that file is not found, default settings will be used. The configuration `toml` 
file consists of three section, `[global]`, `[colors]` and `[keybinds]` each discussed below.

An example configuration file is provided in `./examples/dumpling.toml`, which changes the default colors to the 
[Rose Pine Moon colorscheme](https://rosepinetheme.com/palette/ingredients/). To use it, create the configuration directory and copy the 
`dumpling.toml` file to it, e.g. the following commands in Linux:
```bash
mkdir -p $HOME/.config/dumpling
```
```bash
cp ./examples/dumpling.toml $HOME/.config/dumpling
```
Note: Using this configuration requires a [NerdFont](https://github.com/ryanoasis/nerd-fonts) to be installed.

### Global

Under the `[global]` section, the following can be configured:

| Name | Value | Effect | Default |
|------|-------|--------|---------|
| pdf_viewer | String | PDF viewer to use when attempting to open the paper PDF. | zathura |
| pdf_dir | String | Directory to search for paper PDF files. The `$HOME` and `~` are allowed, even on Windows. See the beginning of the document for the alias expansions. | $HOME/.paper/ |
| selection_icon | String | Characters to put in front of the currently selected paper inside the TUI | -> |
| editor_command | String | Command you want to run to open your preferred file editor on a selected paper information file. The assumed format is `[editor_command] [FILE]`. For terminal editors like Neovim and Vim, make sure you open a new terminal window as illustrated by the default setting when using `kitty`. For editors like VS Code, setting this to `code` should suffice.| `kitty --detach nvim` |

### Colors

Under the `[colors]` section, the following can be configured:
| Name | Effect | Default |
|------|--------|---------|
| master_block_title | Color of the title of the two master blocks named "Paper Explorer" and "Content" | White |
| master_block_border | Color of the border of the two master blocks | White |
| explorer_unselected_fg | Text color of the unselected paper titles | Blue |
| explorer_unselected_bg | Background color of the unselected paper titles | Black |
| explorer_selected_fg | Text color of the selected paper titles | Blue |
| explorer_selected_bg | Background color of the selected paper titles | Gray |
| content_block_title | Color of the title of the content blocks named "Title", "Authors", "Description", "Titles" and "Tags"| White |
| content_block_border | Color of the border of the content blocks | White |
| popup_block_title | Color of the title of a pop-up window | White |
| popup_block_border | Color of the pop-up window border | Red |
| popup_text | Color of the text typed inside pop-up windows | White |
| title_content | Text color of the text inside the "Title" block | White |
| author_content | Text color of the text inside the "Author" block | White |
| description_content | Text color of the text inside the "Description" block | White |
| tag_content | Text color of the text inside the "Tags" block | White |

Note: all colors are assumed to be of the form `[u8,u8,u8]`, representing
RGB values.

### Key binds

Under the `[keybinds]` section, the following can be configured:

| Name | Effect | Default |
|------|--------|---------|
| quit | Exit the TUI | q |
| next | Go to the next paper in the explorer, if the maximum loaded papers is exceeded, the first paper will be unloaded. | j |
| previous | Go to the previous paper in the explorer, if the maximum loaded papers is exceeded, the last paper will be unloaded | k |
| bibtex_to_clipboard | Copy the `bibtex` part of the paper information into the system clipboard. Linux (Wayland and X11), MacOS and Windows are supported, if any errors occur see [cli-clipboard](https://docs.rs/cli-clipboard/latest/cli_clipboard/) | b |
| edit | Open Neovim in a new window with the currently selected paper information file. It is assumed `kitty` and `neovim` are installed. | e |
| delete | Delete the currently selection paper file, it will also be unloaded. A pop-up window will appear asking for confirmation. | d |
| open_in_pdfviewer | Open the PDF file as pointed to by the currenly selected papers `docname` information with the `pdf_reader` set in the `general` section. The `pdf_dir` specified in the `general` section will be searched for this. | o |

Note: all key binds are assumed to be single characters.

## Planned changes

Currently, there are no planned changes.

## Why Dumpling

Because paper -> rice paper -> dumpling.
