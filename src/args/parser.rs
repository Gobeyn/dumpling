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
        "STRING (in triple single quotes)",
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
    opts.optflag("", "version", "Show package information.");
    opts.optflag("h", "help", "Print the help menu.");

    // Parse the arguments options
    let matches = opts.parse(&args[1..]).expect("Error parsing arguments");

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

pub fn print_help(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn print_version() {
    println!("Version: {}", VERSION);
    println!("Package: {}", NAME);
    println!("Author(s): {}", AUTHOR);
    println!("GitHub link: {}", REPOSITORY);
    println!("License: {}", LICENSE);
}
