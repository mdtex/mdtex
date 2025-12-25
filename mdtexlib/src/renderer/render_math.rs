use std::{error::Error, io::Read};
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::Command;

use build_html::{HtmlElement, HtmlTag};
use katex::OutputType;
use std::io::Write;

/*
The plot:
    1. Take math block
    2. Fetch the packages being used
    3. Include \usepackage declarations at the beginning
    4. Compile using either
          a) locally downloaded or
          b) online
       TeX Live compiler
    5. Convert the output into an HTML-friendly format (likely SVG)
    6. Return the output embedded in HTML
    7. Cache the output locally so we don't have to continually re-render
*/

/*
Currently:
    1. Check if user has a valid installation of TeX Live and latexmk
        a) If present, compile blocks locally with TeX Live and latexmk
        b) If not present, return error
    2. Return blocks embedded in HTML
*/

pub fn render_math(math_block: &str, display_mode: bool) -> Result<String, Box<dyn Error>> {
    let opts = katex::Opts::builder()
        .display_mode(display_mode)
        .output_type(OutputType::Mathml)
        .build()
        .unwrap();

    let math = katex::render_with_opts(math_block, &opts)?;
    println!("{}", math);

    Ok(math)
}

pub fn render_inline(math_block: &str, display_mode: bool) -> Result<String, Box<dyn Error>> {
    let opts = katex::Opts::builder()
        .display_mode(display_mode)
        .output_type(OutputType::Mathml)
        .build()
        .unwrap();

    let math = katex::render_with_opts(math_block, &opts)?;
    println!("{}", math);

    Ok(math)
}

// icl gang, this is HORRENDOUS
/// Compile our given latex block to an SVG, which we store in a data directory for later use when we compile our full MDTeX file
pub fn compile_latex_to_svg(math_block: &str, out_directory: &str, filename: &str) -> Result<String, Box<dyn Error>> {
    let mut log_directory = PathBuf::from(format!("{}/logs/", out_directory));
    
    create_dir_all(&out_directory)?;
    create_dir_all(&log_directory)?;

    let mut temp_tex_file = PathBuf::from(format!("{}/temp.tex", out_directory));

    let latex_content = format!(
    r#"\documentclass{{article}}
    
    \pagestyle{{empty}}
    \begin{{document}}
        {}
    \end{{document}}"#,
    math_block);

    // Write LaTeX content to the temporary file
    std::fs::create_dir_all(&out_directory)?;
    std::fs::File::create(&temp_tex_file)?;
    std::fs::write(&temp_tex_file, latex_content.as_bytes())?;

    // What we are essentially executing:
    /*
        #!/usr/bin/env bash
        set -e

        input="$1"
        filename=$(basename "${input%.*}" )

        base="${input%.*}"
        out_dir="$2"

        mkdir -p "$out_dir"
        mkdir -p "${out_dir}/logs"

        pdf_output="${out_dir}/${filename}.pdf"
        svg_output="${out_dir}/${filename}.svg"

        latexmk -pdf -interaction=nonstopmode -halt-on-error "$input" > "${out_dir}/logs/${filename}.log" 2>&1
        pdfcrop "$pdf_output" "$pdf_output"
        pdf2svg "$pdf_output" "$svg_output"
        latexmk -C "$input"

        echo "$svg_output"
    */

    let pdf_out = format!("{}/temp.pdf", &out_directory);

    let mut execute_latexmk = Command::new("latexmk");
    execute_latexmk.args([
        "-pdf",
        "-interaction=nonstopmode",
        "-halt-on-error",
        &format!("-outdir={}", &out_directory),
        &temp_tex_file.to_string_lossy().to_string()
    ]);
    execute_command(&mut execute_latexmk, Some(&log_directory));
    
    let mut execute_pdfcrop = Command::new("pdfcrop");
    execute_pdfcrop.args([&pdf_out, &pdf_out]);
    execute_command(&mut execute_pdfcrop, Some(&log_directory));

    let svg_out = format!("{}/{}.svg", &out_directory, filename);
    let mut execute_pdf2svg = Command::new("pdf2svg");
    execute_pdf2svg.args([&pdf_out, &svg_out]);
    execute_command(&mut execute_pdf2svg, Some(&log_directory))?;

    let mut cleanup = Command::new("latexmk");
    cleanup.args(["-C", &format!("-outdir={}", &out_directory), &pdf_out]);
    execute_command(&mut cleanup, Some(&log_directory))?;

    std::fs::remove_file(temp_tex_file);
    
    Ok(svg_out)
}

/// Execute a given command, and optionally output the results in an out log and an err log
pub fn execute_command(command: &mut Command, log_dir: Option<&PathBuf>) -> Result<String, Box<dyn Error>> {
    let result = command.output()?;

    if let Some(lf) = log_dir {
        let log_folder = lf.to_string_lossy().to_string();

        let out_log = format!("{}/out.log", &log_folder);
        let mut out_log_file = std::fs::OpenOptions::new().write(true).create(true).open(&out_log)?;
        out_log_file.write_all(&result.stdout);

        let err_log = format!("{}/err.log", &log_folder);
        let mut err_log_file = std::fs::OpenOptions::new().write(true).create(true).open(&err_log)?;
        err_log_file.write_all(&result.stderr);
    }

    Ok(format!("{:?}", result))
}