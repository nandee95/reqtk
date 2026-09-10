<details open>
<summary><a id="REQTK-1"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-1</code> <b>ReqTk Requirements</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-2"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-2</code> <b>Commandline interface</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-3"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-3</code> <b>Sub-commands</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-4"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-4</code> <b>convert</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-5"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-5</code> <b>From</b></a></summary>
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
<summary><a id="REQTK-6"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-6</code> <b>To</b></a></summary>
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
<summary><a id="REQTK-7"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-7</code> <b>Span</b></a></summary>
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
<summary><a id="REQTK-8"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-8</code> <b>format</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-9"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-9</code> <b>Format</b></a></summary>
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
</ul>
</details>
<details open>
<summary><a id="REQTK-10"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-10</code> <b>transform</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-12"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-12</code> <b>Id</b></a></summary>
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
<summary><a id="REQTK-13"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-13</code> <b>Prefix</b></a></summary>
<ul>
<blockquote>

Re-apply a prefix to the file.

**arg:** --prefix \<value>
**type:** string

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-14"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-14</code> <b>tokenize</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-15"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-15</code> <b>Format</b></a></summary>
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
<summary><a id="REQTK-16"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-16</code> <b>find</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-17"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-17</code> <b>Id</b></a></summary>
<ul>
<blockquote>

ID of the requirement to find.

**arg:** <id>
**type:** string

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-18"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-18</code> <b>Format</b></a></summary>
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
<summary><a id="REQTK-19"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-19</code> <b>trace</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-20"><img src="REQUIREMENTS/type_informative.svg" />  <code>REQTK-20</code> <b>No additional parameters</b></a></summary>
<ul>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-21"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-21</code> <b>check</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-22"><img src="REQUIREMENTS/type_informative.svg" />  <code>REQTK-22</code> <b>No additional parameters</b></a></summary>
<ul>
</ul>
</details>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-23"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-23</code> <b>Input handling</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-24"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-24</code> <b>Inputs</b></a></summary>
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
<summary><a id="REQTK-25"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-25</code> <b>Output</b></a></summary>
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
<summary><a id="REQTK-26"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-26</code> <b>Help</b></a></summary>
<ul>
<blockquote>

Print help.

**arg:** --help

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-27"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-27</code> <b>Version</b></a></summary>
<ul>
<blockquote>

Print version.

**arg:** --version

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-28"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-28</code> <b>Reporting format</b></a></summary>
<ul>
<blockquote>

Issue reporting format.

**arg:** -r, --report
**type:** enum
**possible values:**
- `human`
- `json` (default)

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-29"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-29</code> <b>Req file format specification</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-30"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-30</code> <b>Syntax</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-82"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-82</code> <b>Root</b></a></summary>
<ul>
<blockquote>

The root of the requirement file may contain any of the following:
- Requirements [![REQTK-79](REQUIREMENTS/badge_REQTK-79.svg)](#REQTK-79)
- File level attributes [![REQTK-80](REQUIREMENTS/badge_REQTK-80.svg)](#REQTK-80) or [![REQTK-81](REQUIREMENTS/badge_REQTK-81.svg)](#REQTK-81)

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-80"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-80</code> <b>Inline attribute definition syntax</b></a></summary>
<ul>
<blockquote>

Inline attributes have the following syntax:
```
[key=value]
```
Where:
`key` - Key of the attribute
`value` - Value of the attribute

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-81"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-81</code> <b>Multi-line attribute definition syntax</b></a></summary>
<ul>
<blockquote>

Multi-line attributes have the following syntax:
```
[key]value[/key]
```
Where:
`key` - Key of the attribute
`value` - Value of the attribute

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-79"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-79</code> <b>Requirement defintion syntax</b></a></summary>
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
`body` - Body of the requirement, may contain any of the following:
	- Another requirement [![REQTK-79](REQUIREMENTS/badge_REQTK-79.svg)](#REQTK-79)
	- An attribute [![REQTK-80](REQUIREMENTS/badge_REQTK-80.svg)](#REQTK-80) or [![REQTK-81](REQUIREMENTS/badge_REQTK-81.svg)](#REQTK-81)

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-88"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-88</code> <b>Requirement type syntax</b></a></summary>
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
- `body` - Body of the requirement type, may contain one of the following:
	- An attribute [![REQTK-80](REQUIREMENTS/badge_REQTK-80.svg)](#REQTK-80) or [![REQTK-81](REQUIREMENTS/badge_REQTK-81.svg)](#REQTK-81)

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-31"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-31</code> <b>Rules</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-32"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-32</code> <b>File level rules</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-33"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-33</code> <b>F001: Attribute key syntax</b></a></summary>
<ul>
<blockquote>

The attribute key must match the following regex: `^[a-z0-9-]+$`

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-34"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-34</code> <b>F002: Duplicate requirement ID</b></a></summary>
<ul>
<blockquote>

A file shall not define the same requirement ID twice.

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-35"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-35</code> <b>F003: Requirement type ID syntax</b></a></summary>
<ul>
<blockquote>

The attribute key must match the following regex: `^[a-z0-9-]+$`

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-36"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-36</code> <b>F004: Duplicate requirement type ID</b></a></summary>
<ul>
<blockquote>

The attribute key must match the following regex: `^[a-z0-9-]+$`

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-78"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-78</code> <b>F005: Requirement ID syntax</b></a></summary>
<ul>
<blockquote>

Requirement ID must not contain a whitespa
ce.

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-89"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-89</code> <b>F006: Invalid attribute value</b></a></summary>
<ul>
<blockquote>

An attribute value must conform to the expected type and constraints. See {@REQTK-39}

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-37"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-37</code> <b>Workspace level rules</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-38"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-38</code> <b>W001: Duplicate requirement id</b></a></summary>
<ul>
<blockquote>

A workspace shall not define the same requirement id twice.

</blockquote>
</ul>
</details>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-39"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-39</code> <b>Built-in attributes</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-40"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-40</code> <b>IDE</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-41"><img src="REQUIREMENTS/type_informative.svg" />  <code>REQTK-41</code> <b>IDE support</b></a></summary>
<ul>
<blockquote>

These attributes are designed for reqtk's own editor. You are allowed to build your own IDE with different attributes.

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-42"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-42</code> <b>File level attributes</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-43"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-43</code> <b>Id prefix</b></a></summary>
<ul>
<blockquote>

Prefix that is auto applied to each identifier.

**key:** id-prefix
**type:** string
**default:** ""

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-44"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-44</code> <b>Id count</b></a></summary>
<ul>
<blockquote>

Counter used to determine the identifier of a newly inserted requirement.

**key:** id-count
**type:** unsigned integer
**default:** 0

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-45"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-45</code> <b>id-type</b></a></summary>
<ul>
<blockquote>

Requirement id type. Used to insert new requirements.

**key:** `d-type
**possible values:**
- `incremental` (default)
- `incremental-N` (where N is an unsigned integer)

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-46"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-46</code> <b>Requirement attributes</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-47"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-47</code> <b>Type</b></a></summary>
<ul>
<blockquote>

Type of the requirement.

**key:** type
**type:** enum
**possible values:**
- One of [![REQTK-88](REQUIREMENTS/badge_REQTK-88.svg)](#REQTK-88)
- One of {@REQTK-52}

**default:** [![REQTK-53](REQUIREMENTS/badge_REQTK-53.svg)](#REQTK-53)

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-48"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-48</code> <b>Description</b></a></summary>
<ul>
<blockquote>

Description of the requirement. Markdown encoded.

**key:** description
**type:** string
**default:** ""

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-49"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-49</code> <b>Requirement type attributes</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-50"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-50</code> <b>Has children</b></a></summary>
<ul>
</ul>
</details>
<details open>
<summary><a id="REQTK-51"><img src="REQUIREMENTS/type_parameter.svg" />  <code>REQTK-51</code> <b>Icon</b></a></summary>
<ul>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-52"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-52</code> <b>Built-in requirement types</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-53"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-53</code> <b>Folder</b></a></summary>
<ul>
<blockquote>

**id:** folder
**title:** Folder
**icon:** F

</blockquote>
</ul>
</details>
<details open>
<summary><a id="REQTK-54"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-54</code> <b>Functional</b></a></summary>
<ul>
</ul>
</details>
<details open>
<summary><a id="REQTK-55"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-55</code> <b>Informative</b></a></summary>
<ul>
</ul>
</details>
<details open>
<summary><a id="REQTK-56"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-56</code> <b>Parameter</b></a></summary>
<ul>
</ul>
</details>
<details open>
<summary><a id="REQTK-57"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-57</code> <b>Limitation</b></a></summary>
<ul>
</ul>
</details>
</ul>
</details>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-58"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-58</code> <b>Others</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-59"><img src="REQUIREMENTS/type_functional.svg" />  <code>REQTK-59</code> <b>Character encoding</b></a></summary>
<ul>
<blockquote>

Req files are UTF-8 encoded.

</blockquote>
</ul>
</details>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-60"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-60</code> <b>Sub-commands</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-61"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-61</code> <b>convert</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-62"><img src="REQUIREMENTS/type_informative.svg" />  <code>REQTK-62</code> <b>Purpose</b></a></summary>
<ul>
<blockquote>

Converts between requirement formats

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-63"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-63</code> <b>format</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-64"><img src="REQUIREMENTS/type_informative.svg" />  <code>REQTK-64</code> <b>Purpose</b></a></summary>
<ul>
<blockquote>

Formats .req documents

</blockquote>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-65"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-65</code> <b>transform</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-66"><img src="REQUIREMENTS/type_informative.svg" />  <code>REQTK-66</code> <b>Purpose</b></a></summary>
<ul>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-67"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-67</code> <b>tokenize</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-68"><img src="REQUIREMENTS/type_informative.svg" />  <code>REQTK-68</code> <b>Purpose</b></a></summary>
<ul>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-69"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-69</code> <b>get</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-70"><img src="REQUIREMENTS/type_informative.svg" />  <code>REQTK-70</code> <b>Purpose</b></a></summary>
<ul>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-71"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-71</code> <b>trace</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-72"><img src="REQUIREMENTS/type_informative.svg" />  <code>REQTK-72</code> <b>Purpose</b></a></summary>
<ul>
</ul>
</details>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-73"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-73</code> <b>File formats</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-74"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-74</code> <b>Req</b></a></summary>
<ul>
<details open>
<summary><a id="REQTK-75"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-75</code> <b>Parsing</b></a></summary>
<ul>
</ul>
</details>
<details open>
<summary><a id="REQTK-76"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-76</code> <b>Tokenizing</b></a></summary>
<ul>
</ul>
</details>
</ul>
</details>
<details open>
<summary><a id="REQTK-77"><img src="REQUIREMENTS/type_folder.svg" />  <code>REQTK-77</code> <b>Json</b></a></summary>
<ul>
</ul>
</details>
</ul>
</details>
</ul>
</details>
