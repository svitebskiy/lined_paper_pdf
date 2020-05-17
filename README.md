# Lined Paper Generator for Handwriting

lined_paper_pdf is a command line utility that generates PDF file with line paper for handwriting.
It takes a YAML file that describes paper size and required line geometry. The input file can request sets of paralled lines (horizontal, vertical, slanted) as well as individual lines.

# Usage

    lined_paper_pdf [OPTIONS] <input-yaml> <output-pdf>

### OPTIONS:
    -n, --num-pages <num-pages>    Number of pages to generate [default: 1]

### ARGS:
    <input-yaml>    Input paper & line set definition YAML file
    <output-pdf>    Output PDF file name