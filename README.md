# simple-html-templating

Simple html templating system for creating static html with a template.

## Usage

The program requires 4 arguments, the paths of:

1. the directory containing text content
2. the template file html file
3. the output directory where templated html will be saved
4. a `context.json` file

The template file must contain `!body!` which is used to replace insert the text content. Each paragraph of the text
content (denoted by two`\n`)
will be wrapped in a `<p></p>` tag. The name of each content file will also be included as a heading.

The context.json file is used to add styling, and should be in the form of:

```json
{
  "ul_css": "",
  "li_css": "",
  "title_css": "",
  "paragraph_css": ""
}
```

## Running

To build the project run:

```bash
cargo build
```

The binary can be run using:

```
cargo run arg1 arg2 arg3 arg4
```

## Testing

Unit tests:

```bash
cargo test --lib
```

Integration tests:

```bash
cargo test --test "*"
```

All:

```bash
cargo test
```


