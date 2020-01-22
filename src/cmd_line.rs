use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;

#[derive(StructOpt, Debug)]
pub struct SlantLineOpt {

}

#[derive(StructOpt, Debug)]
#[structopt(name = "lined_paper_pdf", about = "Generates PDF of lined paper for writing or drawing.")]
pub struct CmdLineOpts {
    /// Input paper & line set definition YAML file
    #[structopt(parse(from_os_str))]
    pub input_yaml: PathBuf,

    /// Output PDF file name
    #[structopt(parse(from_os_str))]
    pub output_pdf: PathBuf,

    /// Number of pages to generate
    #[structopt(short, long, default_value = "1")]
    pub num_pages: u32
}

#[derive(Debug)]
pub enum CmdLine {
    Opts (CmdLineOpts),
    Help (String)
}

pub fn parse_cmd_line() -> Result<CmdLine, Error> {
    let args = std::env::args_os();
    if args.len() < 2 {
        let mut short_help: Vec<u8> = Vec::new();
        let app = CmdLineOpts::clap();
        app.write_help(&mut short_help)?;
        return Ok(CmdLine::Help(String::from_utf8(short_help)?));
    }

    let opt = CmdLineOpts::from_iter_safe(args);
    match opt {
        Ok(opt) => Ok(CmdLine::Opts(opt)),
        Err(e) if e.kind == clap::ErrorKind::HelpDisplayed => {
            let mut long_help: Vec<u8> = Vec::new();
            let app = CmdLineOpts::clap();
            app.write_help(&mut long_help)?;
            Ok(CmdLine::Help(String::from_utf8(long_help)?))
        },
        Err(e) => Err(e.into())
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse command line arguments.")]
    ClapError(#[from] clap::Error),

    #[error("IO Error.")]
    IOError(#[from] std::io::Error),

    #[error("UTF8 verification error.")]
    UTF8Error(#[from] std::string::FromUtf8Error)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::{OsStr, OsString};
    use regex::Regex;

    fn parse<I>(cmd_line: I) -> CmdLineOpts where I: IntoIterator + std::fmt::Debug + Copy, I::Item: Into<OsString> + Clone {
        match CmdLineOpts::from_iter_safe(cmd_line) {
            Ok(opts) => opts,
            Err(e) => panic!("Failed to parse the command line {:?}: {}", cmd_line, e)
        }
    }

    #[test]
    fn args_parse() {
        let cmd_line = ["lined_paper_pdf", "test_line_defs/letter_seyes_slant52.yml", "./test_page_test01.pdf"];
        let opts = parse(&cmd_line);
        assert_eq!(&opts.input_yaml, &OsStr::new("test_line_defs/letter_seyes_slant52.yml"));
        assert_eq!(&opts.output_pdf, &OsStr::new("./test_page_test01.pdf"));
        assert_eq!(opts.num_pages, 1);
    }

    #[test]
    fn help_message_formatting() {
        let mut app = CmdLineOpts::clap();
        let mut msg_bytes: Vec<u8> = Vec::new();

        let usage_rx = Regex::new(r"(?xm)
            ^USAGE:$  \s*
                ^\s+lined_paper_pdf\s\[OPTIONS\]\s<input-yaml>\s<output-pdf>\s*$")
            .unwrap();

        let file_args_rx = Regex::new(r"(?xm)
            ^ARGS:$  \s*
                ^\s+<input-yaml>      \s+Input\spaper\s&\sline\sset\sdefinition\sYAML\sfile\s*
                ^\s+<output-pdf>      \s+Output\sPDF\sfile\sname\s*$")
            .unwrap();

        app.write_help(&mut msg_bytes).expect("Failed to write a short help message.");
        let msg = String::from_utf8(msg_bytes).unwrap();
        println!("Short help text:\n{}", &msg);
        assert!(usage_rx.is_match(&msg), "Usage is present in the short help message.");
        assert!(file_args_rx.is_match(&msg), "Output file is present in the short help message.");

        msg_bytes = Vec::new();
        app.write_long_help(&mut msg_bytes).expect("Failed to write a long help message.");
        let msg = String::from_utf8(msg_bytes).unwrap();
        println!("Long help text:\n{}", &msg);
        assert!(usage_rx.is_match(&msg), "Usage is present in the long help message.");
        assert!(file_args_rx.is_match(&msg), "Output file is present in the long help message.");
    }
}