# Dumpling

Dumpling is a light-weight, minimal research paper manager from the terminal. With the CLI tool, basic information on research papers 
can be created, that can subsequently be explored by the TUI. Inside the TUI a minimal amount of essential actions can be performed. 

By default, the application assumes papers are stored in `$HOME/.paper/`. The paper files created by the CLI tool are stored in the 
`$HOME/.cache/dumpling` directory.

## Installation

For easy installation, a `Makefile` is provided. It is assumed that `GNU Make` and `Rust` are installed on you system. To build the application run:
```
$ make
```
To move the resulting compiled application to the /usr/bin/ directory so it can be accessed globally in you system run:
```
$ make install
```
Lastly, to clean files from the compilation process:
```
$ make clean
```

## Usage 

Dumpling provides a single CLI tool called `dumpling`. The following 
arguments can be attached to it:

| Short form | Long form | Argument | Action |
|------------|-----------|----------|--------|
| -t | --title | "\[TITLE\]" | Set title of paper |
| -y | --year | INT | Set year of publication |
| / | --desc | "\[DESCRIPTION\]" | Short description of the papers contents |
| -b | --bibtex | '''\[BIB\]''' | Bibtex formatted reference for the paper |
| / | --doc | "\[DOCNAME\]" | Name of the PDF document, the directory is set in the configuration file and should not be provided |
| / | --filter-tag | "\[TAG\]" |Show only papers with certain tag. NOTE: this has not been implemented yet.|
| -a | --author | "\[AUTHOR\]" | Add author for the paper, this option can be used multiple times. |
| / | --tag | "\[TAG\]" | Add tag to paper, this option can be used multiple times. |
| -o | --open | No argument | Open the TUI. |
| / | --list-tags | No argument | List all the tags used. NOTE: this has not been implemented yet. |

Note: If you run `dumpling` without any arguments, a default paper 
information entry will be generated. We suggest not using it until we fix 
this behaviour.

Note: The `--filter-tag` and `--list-tag` options have not yet been implemented.

## Configuration

The user can create their own configuration for certain elements of the program with a configuration file. When starting the program, it will search 
for the presence of `$HOME/.config/dumpling/dumpling.toml`, in case that file is not found, default settings will be used. The configuration `toml` 
file consists of three section 

### Global

Under the `[global]` section, the following can be configured:

| Name | Value | Effect | Default |
|------|-------|--------|---------|
| load_size | 32-bit integer | Amount of files loaded when TUI is opened. | 30 |
| pdf_viewer | String | PDF viewer to use when attempting to open the paper PDF. | zathura |
| pdf_dir | String | Directory to search for paper PDF files. Aliases such a `$HOME` and `~` are not supported, so direct paths must be provided | $HOME/.paper/ |
| selection_icon | Char | Single character to put in front of the currently selected paper inside the TUI |   |

### Colors

Under the `[colors]` section, the following can be configured:
| Name | Effect | Default |
|------|--------|---------|
| master_block_title | Color of the title of the two master blocks named "Paper Explorer" and "Content" | White |
| master_block_border | Color of the border of the two master blocks | White |
| explorer_unselected_fg | Text color of the unselected paper titles | Blue |
| explorer_unselected_bg | Background color of the unselected paper titles | Black |\
| explorer_selected_fg | Text color of the selected paper titles | Blue |
| explorer_selected_bg | Background color of the selected paper titles | Gray |
| content_block_title | Color of the title of the three content blocks named "Title", "Authors" and "Description" | White |
| content_block_border | Color of the border of the three content blocks | White |
| title_content | Text color of the text inside the "Title" block | White |
| author_content | Text color of the text inside the "Author" block | White |
| description_content | Text color of the text inside the "Description" block | White |

Note: all colors are assumed to be of the form `[u8,u8,u8]`, representing
RGB values.

### Key binds

Under the `[keybinds]` section, the following can be configured:

| Name | Effect | Default |
|------|--------|---------|
| quit | Exit the TUI | q |
| next | Go to the next paper in the explorer, if the maximum loaded papers is exceeded, the first paper will be unloaded. | j |
| previous | Go to the previous paper in the explorer, if the maximum loaded papers is exceeded, the last paper will be unloaded | k |
| bibtex_to_clipboard | Copy the `bibtex` part of the paper information into the system clipboard. Currently, `wl-clipboard` is used and nothing else is supported | b |
| edit | Open Neovim in a new window with the currently selected paper information file. It is assumed `kitty` and `neovim` are installed. | e |
| delete | Delete the currently selection paper file, it will also be unloaded. However, the PDF for it will not be deleted. | d |
| open_in_pdfviewer | Open the PDF file as pointed to by the currenly selected papers `docname` information with the `pdf_reader` set in the `general` section. The `pdf_dir` specified in the `general` section will be searched for this. | o |

Note: all key binds are assumed the be single characters.
