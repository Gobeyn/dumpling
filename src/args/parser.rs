use super::super::file;

const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const LICENSE: &str = env!("CARGO_PKG_LICENSE");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

/// Summary of boolean program arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgFlags {
    pub open: bool,
    pub list_tags: bool,
    pub pdf_diagnostic: bool,
    pub auto: bool,
}

/// Program arguments contained in a single structure, including
/// boolean flags, which are summarised by `ProgFlags`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgArgs {
    pub title: String,
    pub year: i32,
    pub description: String,
    pub bibtex: String,
    pub docname: String,
    pub authors: Vec<String>,
    pub tags: Vec<String>,
    pub flags: ProgFlags,
    pub filter_by_tag: String,
}

impl Default for ProgFlags {
    fn default() -> Self {
        ProgFlags {
            open: false,
            list_tags: false,
            pdf_diagnostic: false,
            auto: false,
        }
    }
}

impl Default for ProgArgs {
    fn default() -> Self {
        ProgArgs {
            title: String::new(),
            year: 0,
            description: String::new(),
            bibtex: String::new(),
            docname: String::new(),
            authors: Vec::new(),
            tags: Vec::new(),
            flags: ProgFlags::default(),
            filter_by_tag: String::new(),
        }
    }
}

impl ProgArgs {
    /// Check if struct is any different from the default.
    pub fn is_default(&self) -> bool {
        if *self == Self::default() {
            return true;
        } else {
            return false;
        }
    }

    /// Convert subset of `ProgArgs` fields into `Paper` struct.
    pub fn to_paper(&self) -> Option<file::parser::Paper> {
        // Check of the program arguments are any different from the default, i.e. if the user
        // specified any fields.
        if self.is_default() {
            return None;
        }

        // Format the authors and tags
        let mut author_vec: Vec<file::parser::Author> = Vec::new();
        let mut tag_vec: Vec<file::parser::Tag> = Vec::new();

        for val in self.authors.iter() {
            let author: file::parser::Author = file::parser::Author { name: val.clone() };
            author_vec.push(author);
        }
        for val in self.tags.iter() {
            let tag: file::parser::Tag = file::parser::Tag { label: val.clone() };
            tag_vec.push(tag);
        }

        // Take subset from the program arguments and pass them to the paper.
        let paper = file::parser::Paper {
            title: self.title.clone(),
            year: self.year,
            description: self.description.clone(),
            bibtex: self.bibtex.clone(),
            docname: self.docname.clone(),
            authors: author_vec,
            tags: tag_vec,
        };
        return Some(paper);
    }
}

/// Using the `getopts` crate, the program arguments are parsed into the
/// `ProgArgs` struct. Any arguments not provided take a default value
/// as described by the Default implementation on `ProgArgs`.
pub fn parse_arguments() -> ProgArgs {
    let args: Vec<String> = std::env::args().collect();
    let mut prog_args: ProgArgs = ProgArgs::default();
    let mut opts = getopts::Options::new();
    let program = args[0].clone();

    // Define the program options
    opts.optopt("t", "title", "Title of paper", "STRING (in double quotes)");
    opts.optopt("y", "year", "Year of paper publication", "INTEGER (0-?)");
    opts.optopt(
        "",
        "desc",
        "Short description of the contents",
        "STRING (in double quotes)",
    );
    opts.optopt(
        "b",
        "bibtex",
        "Bibtex bibliography for paper",
        "STRING (in double quotes)",
    );
    opts.optopt(
        "",
        "doc",
        "Name under which paper is saved. All papers are assumed to be stored in $HOME/.paper/.",
        "STRING (in double quotes)",
    );
    opts.optopt(
        "",
        "filter-tag",
        "Filter the papers by a tag. This only does something if the TUI is opened.",
        "STRING (in double quotes)",
    );

    // Multi opts
    opts.optmulti(
        "a",
        "author",
        "Author(s) of the paper. If there are multiple authors, this flag can be used multiple times.",
        "STRING (in double quotes)",
    );
    opts.optmulti(
        "",
        "tag",
        "Tag to attach to the paper. A paper can have multiple tags by using this flag multiple times.",
        "STRING (in double quotes)",
    );

    // Boolean flags
    opts.optflag("o", "open", "Open the TUI.");
    opts.optflag("", "list-tags", "Print all the tags used to the terminal.");
    opts.optflag("", "pdf-diagnose", "Show the file paths to all the invalid PDF links in the paper files and all the unused existing PDF files.");
    opts.optflag(
        "",
        "auto",
        "Use bibtex String to fill in the title, year and author fields.",
    );
    opts.optflag("", "version", "Show package information.");
    opts.optflag("h", "help", "Print the help menu.");

    // Parse the arguments options
    let matches = opts.parse(&args[1..]).expect("Error parsing arguments");

    // Auto needs to be done first, so it can be overwritten by
    // explicit title, year and author arguments.
    if matches.opt_present("auto") {
        // This requires bibtex to be present
        if matches.opt_present("b") {
            // Parse the bibtex string
            let bibtex_str = matches.opt_str("b").expect("Error with --bibtex");
            let (title, year, authors) = extract_bibtex_fields(&bibtex_str);
            match title {
                Some(t) => {
                    prog_args.title = t.clone();
                    println!("Title: {}", t);
                }
                None => {}
            }
            match year {
                Some(y) => {
                    prog_args.year = y;
                    println!("Year: {}", y);
                }
                None => {}
            }
            match authors {
                Some(a) => {
                    prog_args.authors = a.clone();
                    println!("Authors: {:?}", a);
                }
                None => {}
            }
        }
    }

    // Check if title is present
    if matches.opt_present("t") {
        prog_args.title = matches.opt_str("t").expect("Error with --title");
    }
    // Check if year is present
    if matches.opt_present("y") {
        let res = matches.opt_str("y").expect("Error with --year");
        prog_args.year = res
            .parse::<i32>()
            .expect("Error parsing --year into integer");
    }
    // Check if description is present
    if matches.opt_present("desc") {
        prog_args.description = matches.opt_str("desc").expect("Error with --description");
    }
    // Check if bibtex is present
    if matches.opt_present("b") {
        prog_args.bibtex = matches.opt_str("b").expect("Error with --bibtex");
    }
    // Check if document-name is present
    if matches.opt_present("doc") {
        prog_args.docname = matches.opt_str("doc").expect("Error with --doc-name");
    }
    // Check if filter-tag is present
    if matches.opt_present("filter-tag") {
        prog_args.filter_by_tag = matches
            .opt_str("filter-tag")
            .expect("Error with --filter-tag");
    }
    // Multi opts
    // Check if authors were provided
    if matches.opt_present("a") {
        prog_args.authors = matches.opt_strs("a");
    }
    // Check if tags were provided
    if matches.opt_present("tag") {
        prog_args.tags = matches.opt_strs("tag");
    }

    // Boolean opts
    // Check if open is flagged
    if matches.opt_present("o") {
        prog_args.flags.open = !prog_args.flags.open;
    }
    // Check if listing tags is flagged
    if matches.opt_present("list-tags") {
        prog_args.flags.list_tags = !prog_args.flags.list_tags;
    }
    if matches.opt_present("pdf-diagnose") {
        prog_args.flags.pdf_diagnostic = !prog_args.flags.pdf_diagnostic;
    }
    if matches.opt_present("version") {
        print_version();
    }
    if matches.opt_present("h") {
        print_help(&program, opts);
    }
    return prog_args;
}

/// Using the information provided by the `Options`, print a help message to the terminal for all
/// the options and how they should be used.
pub fn print_help(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}
/// Print some program information to the terminal.
pub fn print_version() {
    println!("Version: {}", VERSION);
    println!("Package: {}", NAME);
    println!("Author(s): {}", AUTHOR);
    println!("GitHub link: {}", REPOSITORY);
    println!("License: {}", LICENSE);
}
/// Given a bibtex format string, extract from it the title, year and authors. If any of these
/// extractions fail, None is return for that field.
pub fn extract_bibtex_fields(
    bibtex: &String,
) -> (Option<String>, Option<i32>, Option<Vec<String>>) {
    // Define the regular expression that will extract each fields
    let title_re =
        regex::Regex::new(r"(?i)title\s*=\s*\{([^}]+)\}").expect("Error generating regex.");
    let year_re =
        regex::Regex::new(r"(?i)year\s*=\s*\{([^}]+)\}").expect("Error generating regex.");
    let author_re =
        regex::Regex::new(r"(?i)author\s*=\s*\{([^}]+)\}").expect("Error generating regex.");

    // Get the matched title, None if there was no match.
    let title: Option<String> = {
        if title_re.is_match(bibtex) {
            let caps = title_re.captures(bibtex);
            match caps {
                Some(c) => match c.get(1) {
                    Some(s) => Some(s.as_str().to_string()),
                    None => None,
                },
                None => None,
            }
        } else {
            None
        }
    };
    // Get the matched year, None if there was no match.
    let year: Option<i32> = {
        if year_re.is_match(bibtex) {
            let caps = year_re.captures(bibtex);
            match caps {
                Some(c) => match c.get(1) {
                    Some(s) => Some(s.as_str().parse::<i32>().unwrap()),
                    None => None,
                },
                None => None,
            }
        } else {
            None
        }
    };
    // Get the authors string
    let authors_str: Option<String> = {
        if author_re.is_match(bibtex) {
            let caps = author_re.captures(bibtex);
            match caps {
                Some(c) => match c.get(1) {
                    Some(s) => Some(s.as_str().to_string()),
                    None => None,
                },
                None => None,
            }
        } else {
            None
        }
    };
    // We assume the authors field follows to expected format:
    // author={Doe, John and Smith, Jane and ...} and extract from
    // that ["John Doe", "Jane Smith", ...]
    let authors: Option<Vec<String>> = match authors_str {
        Some(s) => Some(
            // Split into ["Doe, John", "Smith, Jane", ...]
            s.split(" and ")
                // For each name (e.g. Doe, John) split into first and last name (e.g. ["Doe", "John"])
                .map(|name| {
                    let parts: Vec<&str> = name.split(',').map(|s| s.trim()).collect();
                    if parts.len() == 2 {
                        format!("{} {}", parts[1], parts[0])
                    } else {
                        // If the name does not fit the expected format, just use it as is.
                        name.to_string()
                    }
                })
                .collect(),
        ),
        None => None,
    };

    // Return the parsed fields
    return (title, year, authors);
}
