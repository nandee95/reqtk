<details open>
<summary><a id="REQTK-1"><img src="REQUIREMENTS/type_limitation.svg" />  <code>REQTK-1</code> <b>Work in progress</b></a></summary>
<ul>
<blockquote>

This document is a work in progress and may not cover all aspects of the requirements. The identifiers, structure and content is subject to change.

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-2"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-2</code> <b>ReqTk Requirements</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-3"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-3</code> <b>Commandline interface</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-4"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-4</code> <b>Sub-commands</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-5"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-5</code> <b>convert</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-6"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-6</code> <b>From</b></a></summary>
<ul>
<blockquote>

Input format (defaults to extension of <input> if auto).

**arg:** -f <value>, --from <value> \
**possible values:**
- `auto` (default)
- `req`
- `json`

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-7"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-7</code> <b>To</b></a></summary>
<ul>
<blockquote>

Output format (defaults to extension of --output if auto).

**arg:** -t <value>, --to <value>\
**possible values:**
- `auto` (default)
- `req`
- `json`
- `md`

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-8"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-8</code> <b>Span</b></a></summary>
<ul>
<blockquote>

Include spans (for json format only).

**arg:** --span

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-9"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-9</code> <b>format</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-10"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-10</code> <b>Format</b></a></summary>
<ul>
<blockquote>

Formatting style to apply.

**arg:** -f <value>, --format <value>
**possible values:**
- `reformat` (default)
- `minify`
- `unchanged`

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-11"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-11</code> <b>Diff</b></a></summary>
<ul>
<blockquote>

Show differences instead of writing to the output(s).

**arg:** --diff

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-12"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-12</code> <b>transform</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-13"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-13</code> <b>Id</b></a></summary>
<ul>
<blockquote>

Re-apply ids for the file.

**arg:** --id <value>
**possible values:**
- `incremental`
- `incremental-N` (where N is an unsigned integer)

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-14"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-14</code> <b>Id prefix</b></a></summary>
<ul>
<blockquote>

Re-apply ID prefix to the file.

**arg:** --id-prefix \<value>
**type:** string

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-15"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-15</code> <b>tokenize</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-16"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-16</code> <b>Format</b></a></summary>
<ul>
<blockquote>

Output format.

**arg:** -f <value>, --format <value>
**possible values:**
- `human` (default)
- `json`

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-17"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-17</code> <b>find</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-18"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-18</code> <b>Id</b></a></summary>
<ul>
<blockquote>

ID of the requirement to find.

**arg:** <id>
**type:** string

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-19"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-19</code> <b>Format</b></a></summary>
<ul>
<blockquote>

Output format.

**arg:** -f <value>, --format <value>
**possible values:**
- `req` (default)
- `json`

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-20"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-20</code> <b>trace</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-21"><img src="REQUIREMENTS/type_informative.svg" />  <code>REQTK-21</code> <b>No additional parameters</b></a></summary>
<ul>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-22"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-22</code> <b>check</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-23"><img src="REQUIREMENTS/type_informative.svg" />  <code>REQTK-23</code> <b>No additional parameters</b></a></summary>
<ul>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-24"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-24</code> <b>init</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-25"><img src="REQUIREMENTS/type_informative.svg" />  <code>REQTK-25</code> <b>No additional parameters</b></a></summary>
<ul>
</ul>
</details>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-26"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-26</code> <b>Input handling</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-27"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-27</code> <b>Inputs</b></a></summary>
<ul>
<blockquote>

Input locations. When missing the inputs are taken from nearest 'reqtk.json'.
Use a single '-' for stdin.

**arg:** <input>
**type:** path(s)

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-28"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-28</code> <b>Output</b></a></summary>
<ul>
<blockquote>

Output location for single input.
Use '-' for stdout.

**arg:** -o <value>, --output <value>
**type:** path

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-29"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-29</code> <b>Help</b></a></summary>
<ul>
<blockquote>

Print help.

**arg:** --help

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-30"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-30</code> <b>Version</b></a></summary>
<ul>
<blockquote>

Print version.

**arg:** --version

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-31"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-31</code> <b>Reporting format</b></a></summary>
<ul>
<blockquote>

Issue reporting format.

**arg:** -r, --report
**type:** enum
**possible values:**
- `human` (default)
- `json`

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-32"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-32</code> <b>Verbose</b></a></summary>
<ul>
<blockquote>

Enable verbose output.

**arg:** -v, --verbose

</blockquote>
</ul>
</details>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-33"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-33</code> <b>Req file specification</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-34"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-34</code> <b>Syntax</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-35"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-35</code> <b>Root</b></a></summary>
<ul>
<blockquote>

The root of the requirement contains zero or more items of the following list, in any order, duplications allowed:
- [![REQTK-40](REQUIREMENTS/badge_REQTK-40.svg)](#REQTK-40)
- [![REQTK-39](REQUIREMENTS/badge_REQTK-39.svg)](#REQTK-39)
- [![REQTK-36](REQUIREMENTS/badge_REQTK-36.svg)](#REQTK-36)

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-36"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-36</code> <b>Attribute syntax</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-37"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-37</code> <b>Inline attribute syntax</b></a></summary>
<ul>
<blockquote>

Inline attributes have the following syntax:
```
[key=value]
```
Where:
`key` - Key of the attribute (`=`, `]` characters shall be escaped, see [![REQTK-41](REQUIREMENTS/badge_REQTK-41.svg)](#REQTK-41))
`value` - Value of the attribute (`]` character shall be escaped, see [![REQTK-41](REQUIREMENTS/badge_REQTK-41.svg)](#REQTK-41))

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-38"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-38</code> <b>Multi-line attribute syntax</b></a></summary>
<ul>
<blockquote>

Multi-line attributes have the following syntax:
```
[key]value[/key]
```
Where:
`key` - Key of the attribute (`=`, `]` characters shall be escaped, see [![REQTK-41](REQUIREMENTS/badge_REQTK-41.svg)](#REQTK-41))
`value` - Value of the attribute (`[` character shall be escaped, see [![REQTK-41](REQUIREMENTS/badge_REQTK-41.svg)](#REQTK-41))

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-39"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-39</code> <b>Requirement syntax</b></a></summary>
<ul>
<blockquote>

Requirement definition has the following syntax:
```
@id(title) {
	body
}
```
Where:
`id` - Identifier of the requirement
`title` - Title of the requirement
`body` - Body of the requirement, may contain zero or more items from the following list, in any order, duplications allowed:
	- [![REQTK-39](REQUIREMENTS/badge_REQTK-39.svg)](#REQTK-39)
	- [![REQTK-36](REQUIREMENTS/badge_REQTK-36.svg)](#REQTK-36)

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-40"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-40</code> <b>Requirement type syntax</b></a></summary>
<ul>
<blockquote>

Requirement type has the following syntax:
```
$id(title) {
	body
}
```
Where:
- `id` - Identifier of the requirement type
- `title` - Title of the requirement type
- `body` - Body of the requirement type, may contain zero or more items from the following list, in any order, duplications allowed:
	- [![REQTK-36](REQUIREMENTS/badge_REQTK-36.svg)](#REQTK-36)

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-41"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-41</code> <b>Escaping</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-42"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-42</code> <b>Escape syntax</b></a></summary>
<ul>
<blockquote>

Any character may be escaped.
An escaped character means a literal character that shall not be interpreted as a syntactic token. (`\`). (e.g. `=` => `\=`, `]` => `\]`)

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-43"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-43</code> <b>Using the escape character</b></a></summary>
<ul>
<blockquote>

To include the escape character itself, it shall be escaped with another backslash (e.g. `\` => `\\`).

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-44"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-44</code> <b>Whitespaces</b></a></summary>
<ul>
<blockquote>

Whitespaces are allowed anywhere in the req file.

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-45"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-45</code> <b>Character encoding</b></a></summary>
<ul>
<blockquote>

The req file shall support UTF-8 character encoding.

</blockquote>
</ul>
</details>
</ul>
</details>
