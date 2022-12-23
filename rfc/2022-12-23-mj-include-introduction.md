# RFC Introduction of the `mj-include` component

The goal of this documentation is to explain how the `mj-include` will be implemented. It will take in consideration the current architecture, the different steps when processing a template and how the `mj-include` should plug.

## Current architecture and process flow

The current workflow works as follows
- call `to_html("/path/to/my-file.mjml")`
- the `/path/to/my-file.mjml` file is converted to a `String`
- it is then parsed with a `xml` parser
- the template is then converted into a `MJML` element using the `Parser` trait
- the obtained `MJML` element can then be converted (rendered) to `JSON` or to `HTML` depending on what the user wants.

The goal of having a separating between the `Parse` and `Render` step is to be able to save the intermediate state.
In the current state, it's possible to parse a mjml template, save its `JSON` representation and reuse it later to render it.

## Requirements

Let's consider the following example

```xml
<!-- main.mjml -->
<mjml>
  <mj-head>
    <mj-title>test with mj-include</mj-title>
  </mj-head>
  <mj-body>
    <mj-include path="./partial.mjml" />
    <mj-include path="file://path/to/partial.html" type="html"/>
    <mj-include path="https://example.com/path/to/partial.html" type="html"/>
  </mj-body>
</mjml>
<!-- partial.mjml -->
<mj-button>A nice button!</mj-button>
<!-- partial.html -->
<div>Just a div!</div>
```

Should be the equivalent of

```xml
<mjml>
  <mj-head>
    <mj-title>test with mj-include</mj-title>
  </mj-head>
  <mj-body>
    <mj-button>A nice button!</mj-button>
    <mj-raw>
      <div>Just a div!</div>
    </mj-raw>
    <mj-raw>
      <div>Just a div!</div>
    </mj-raw>
  </mj-body>
</mjml>
```

For the `JSON` representation, there are two possibilities.

The first option is be to replace the `mj-include` by its content, obtaining the equivalent representation.

The second option is to *just* use the `mj-include` as a component and not load nor validate the content of the `mj-include` template.

## Consideration

The `mj-include` should be allowed to load the templates from any kind of source, from an url, from the local path, relative or not.

## Implementation
