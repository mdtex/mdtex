Projected cli structure:

### Basic cmds
---
##### Compile
Compilation executed using `mdtex` binary:
- `mdtex compile <flags> <input-file>.mdt`

###### Flags
- Flags to customize the potential output
- `--recursive`, `-r <input-folder>`
- `--output`, `-o <output-file>.*`
  - Specify the output file - should be html or pdf
  - HTML by default
- `--write-recursive`, `-wr <output-folder>`
  - Specify the output folder
- `--html`
  - Output the given input file(s) as a/n html file(s)
- `--pdf`
  - Output the given input file(s) as a/n pdf(s)

##### Watch
- Watch a particular file and live-compile it:
- `mdtex watch <flags> <input-file>.mdt`

### Manage projects
---
##### Init
- Initialize a new mdtex project
- `mdtex init <flags> <project-name>`

##### Add
- Add a LaTeX package to the project
- `mdtex add <flags> <package-name>`

##### Build project
- Build the whole mdtex project
- `mdtex build <flags>`

TODO: Add more flags for everything, clean up instruction descriptions and hierarchy