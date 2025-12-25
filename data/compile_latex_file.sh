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