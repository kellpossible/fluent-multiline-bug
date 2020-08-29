# Minimal Example for [Fluent Issue #191](https://github.com/projectfluent/fluent-rs/issues/191)

This is a minimal reproduction of the issue <https://github.com/projectfluent/fluent-rs/issues/191>.

The expected successful output of the program is the following:

```txt
Contents of test.ftl:

simple-message = Hello
multiline-message = Hello this is
    a multiline message
    using fluent.
multiline-message-selector = { $size ->
        [big] Big
        *[medium] Medium
        [small] Small
    }
simple-arg = Hello {$name}

Successfully found entry: Entry(Message(Message { id: Identifier { name: "simple-message" }, value: Some(Pattern { elements: [TextElement("Hello")] }), attributes: [], comment: None }))
Successfully found entry: Entry(Message(Message { id: Identifier { name: "simple-arg" }, value: Some(Pattern { elements: [TextElement("Hello "), Placeable(InlineExpression(VariableReference { id: Identifier { name: "name" } }))] }), attributes: [], comment: None }))
Successfully found entry: Entry(Message(Message { id: Identifier { name: "multiline-message" }, value: Some(Pattern { elements: [TextElement("Hello this is\n"), TextElement("a multiline message\n"), TextElement("using fluent.")] }), attributes: [], comment: None }))
Successfully found entry: Entry(Message(Message { id: Identifier { name: "multiline-message-selector" }, value: Some(Pattern { elements: [Placeable(SelectExpression { selector: VariableReference { id: Identifier { name: "size" } }, variants: [Variant { key: Identifier { name: "big" }, value: Pattern { elements: [TextElement("Big")] }, default: false }, Variant { key: Identifier { name: "medium" }, value: Pattern { elements: [TextElement("Medium")] }, default: true }, Variant { key: Identifier { name: "small" }, value: Pattern { elements: [TextElement("Small")] }, default: false }] })] }), attributes: [], comment: None }))
```

To reproduce the issue, ensure that the `test.ftl` is saved as CRLF. On unix, by default it is LF.

The expected output when the error occurs is the following:

```txt
Contents of test.ftl:

simple-message = Hello
multiline-message = Hello this is
    a multiline message
    using fluent.
multiline-message-selector = { $size ->
        [big] Big
        *[medium] Medium
        [small] Small
    }
simple-arg = Hello {$name}

The following errors occurred while parsing test.ftl:
[ParserError { pos: (142, 143), slice: Some((103, 219)), kind: ExpectedToken('\n') }]
Successfully found entry: Entry(Message(Message { id: Identifier { name: "simple-message" }, value: Some(Pattern { elements: [TextElement("Hello")] }), attributes: [], comment: None }))
Successfully found entry: Entry(Message(Message { id: Identifier { name: "simple-arg" }, value: Some(Pattern { elements: [TextElement("Hello "), Placeable(InlineExpression(VariableReference { id: Identifier { name: "name" } }))] }), attributes: [], comment: None }))
Successfully found entry: Entry(Message(Message { id: Identifier { name: "multiline-message" }, value: Some(Pattern { elements: [TextElement("Hello this is"), TextElement("\n"), TextElement("a multiline message"), TextElement("\n"), TextElement("using fluent.")] }), attributes: [], comment: None }))
Error: Could not find entry matching id `multiline-message-selector`
```

As you can see, it managed to find all the entries, except the `multiline-message-selector`, which had earlier triggered a `ParseError`.
