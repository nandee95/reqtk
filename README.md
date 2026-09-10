[![Crates.io](https://img.shields.io/crates/v/reqtk.svg)](https://crates.io/crates/reqtk)
[![Downloads](https://img.shields.io/crates/d/reqtk.svg)](https://crates.io/crates/reqtk)
[![Docs](https://docs.rs/reqtk/badge.svg)](https://docs.rs/reqtk/latest/reqtk/)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE.md)
[![Requirements](https://img.shields.io/badge/requirements-REQ-blue)](REQUIREMENTS.md)

# Req Toolkit

ReqTk is a requirements management toolkit built around a human-readable, git-friendly `.req` file format. Define, validate, format, trace, transform, and convert requirements directly from your repository.

> ⚠️Experimental! \
> ReqTk is still under development and is missing some
> critical tests. Data loss is possible. Keep backups and report bugs.

VSCode extension is coming soon!

## Getting started

1. Install `cargo` ([instructions](https://doc.rust-lang.org/cargo/getting-started/installation.html))
2. Run `cargo install reqtk`
3. Navigate to your project's root directory.
4. Run `reqtk init`
    - This will create `reqtk.json` and `requirements.req`.
    - Adjust the source and requirement file paths to your needs.

`init` will generate the following req file:
```
[id-type=incremental]
[id-prefix=REQ-]
[id-count=2]
@REQ-1(My requirement) {
	[type=functional]
	[description]
	Hello world!
	[/description]
}
```
### Generate markdown
You can generate markdown from your requirements using: \
`reqtk convert -o REQUIREMENTS.md`

### Reflow ids in your requirement file
Before first commit you might want to re-assign the ids in your docment incrementally.

## ReqTk usage

```
ReqTk is a requirements management toolkit built around a human-readable, git-friendly `.req` file format.

Usage: reqtk [OPTIONS] <COMMAND>

Commands:
  format     Format req files
  check      Analyze req files for issues
  convert    Convert between known requirement formats (req, json)
  transform  Transform a req file (e.g. reflow ids, minify)
  tokenize   Tokenize a req file (output: JSON)
  trace      Find traces for requirements (output: JSON)
  find       Finds a single requirement (output: JSON)
  init       Initializes a reqtk.json workspace file
  help       Print this message or the help of the given subcommand(s)

Options:
  -r, --report <REPORT>  Issue reporting format [default: human] [possible values: human, json]
  -v, --verbose          Enable verbose output
  -h, --help             Print help
  -V, --version          Print version
```


# REQ file format

## Design goals
- Easy to review (human-readable, good git diffs)
- Hierarchical structure
- History tied to the source (via git)
- Possibility for custom attributes (e.g. multiple-languages, tags, etc.)
- IDE support possibility
- AI friendly (e.g. tooling to search requirements)
- Conversion from existing requirement formats
- Easy way to publish for open-source projects (markdown conversion)
- Tracing (to other requirements, to tests, to issues)
- CI support (format, validate)

## Req file elements

### Attributes

Single line attribute:
```
[key=value]
```
Multi-line attribute:
```
[key]
multi
line
value
[/key]
```
Attributes can be placed at root level or inside other elements (see later).

Root level attributes:
| key | type | notes |
| - | - | - |
| id-type | `incremental`, `incremental-N`, `random`, `random-N`, where N is an integer | Type of the requirement IDs. Used by IDE's to generate the identifiers for new requirements. |
| id-prefix | string | Prefix of the requirement IDs. Type of the requirement IDs. Used by IDE's to generate the identifiers for new requirements. |
| id-count | unsigned integer | Only used when `id-type`=`incremental*`. Contains the counter for incremental requirement IDs. Used by IDE's to generate the identifiers for new requirements. |

### Requirement types
You can define your own requirement types with the following syntax (root level):
```
$custom(Custom) {
    [icon=star]
    [color=yellow]
    [has-children=true]
}
```

Requirement type attributes:
| key | type | notes |
| - | - | - |
| icon | [material icon](https://fonts.google.com/icons?icon.set=Material+Icons&icon.style=Filled) | Icon of the requirement type. |
| color | html/css color | Color of the icon. |
| has-children | `true`/`false` | Allow the IDE to add children to the requirement. |


The following requirement types are built-in:
| icon | id | title | has-children | notes |
| - | - | - | - |- |
| ![folder](https://api.iconify.design/ic/baseline-folder.svg?color=%23facc15) | folder | Folder | true | (default) |
| ![bolt](https://api.iconify.design/ic/baseline-bolt.svg?color=%233b82f6) | functional | Functional | false | |
| ![article](https://api.iconify.design/ic/baseline-article.svg?color=%23a855f7) | informative | Informative | false | |
| ![tag](https://api.iconify.design/ic/baseline-tag.svg?color=%2322c55e) | parameter | Parameter | false | |
| ![warning](https://api.iconify.design/ic/baseline-warning.svg?color=%23ef4444) | limitation | Limitation | false | |


### Requirements
You can add requirements with the following syntax:
```
@parent(Parent) {
  @child(Child) {
    [type=functional]
    [description]
    Hello world!
    [/description]
  }
}
```

Requirement attributes:
| key | type | notes |
| - | - | - |
| type | `folder` (default), `functional`, `informative`, `parameter`, `limitation` or the identifier of a custom requirement type | Requirement type. |
| description | markdown text | Requirement description. |


The requirement description supports references to other requirements in the following format:
```
@REQ-1(First) {
  [type=functional]
  [description]
  This is a reference to @REQ-2
  [/description]
}
@REQ-2(Second) {
  [type=functional]
  [description]
  This is a reference to @REQ-1
  [/description]
}
```

The traces will show up in the `reqtk trace` command.