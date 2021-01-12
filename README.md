# Minimal Example for [Fluent Issue #191](https://github.com/projectfluent/fluent-rs/issues/191)

This is a minimal reproduction of the issue <https://github.com/projectfluent/fluent-rs/issues/191>.

The expected successful output of the program is the following:

```txt
Contents of test.ftl:

multiline-message = Hello this is
    a multiline message
    using fluent.
multiline-message-break =
    Hello this is
    a multiline
    message.
# No spaces/tabbing on the empty lines.
multiline-message-breaks =
    Hello this is

    a multiline

    message.
# Tabbing/spaces on the empty lines.
multiline-message-breaks-spaces =
    Hello this is
    
    a multiline
    
    message.
# No spaces/tabbing on the empty lines.
multiline-message-args =
    This is a multiline message.

    { $argOne }

    This is a multiline message.

    { $argTwo }

Successfully found entry: Message(
    Message {
        id: Identifier {
            name: "multiline-message",
        },
        value: Some(
            Pattern {
                elements: [
                    TextElement {
                        value: "Hello this is\n",
                    },
                    TextElement {
                        value: "a multiline message\n",
                    },
                    TextElement {
                        value: "using fluent.",
                    },
                ],
            },
        ),
        attributes: [],
        comment: None,
    },
)
Successfully found entry: Message(
    Message {
        id: Identifier {
            name: "multiline-message-break",
        },
        value: Some(
            Pattern {
                elements: [
                    TextElement {
                        value: "Hello this is\n",
                    },
                    TextElement {
                        value: "a multiline\n",
                    },
                    TextElement {
                        value: "message.",
                    },
                ],
            },
        ),
        attributes: [],
        comment: None,
    },
)
Successfully found entry: Message(
    Message {
        id: Identifier {
            name: "multiline-message-breaks",
        },
        value: Some(
            Pattern {
                elements: [
                    TextElement {
                        value: "Hello this is\n",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "a multiline\n",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "message.",
                    },
                ],
            },
        ),
        attributes: [],
        comment: Some(
            Comment {
                content: [
                    "No spaces/tabbing on the empty lines.",
                ],
            },
        ),
    },
)
Successfully found entry: Message(
    Message {
        id: Identifier {
            name: "multiline-message-breaks-spaces",
        },
        value: Some(
            Pattern {
                elements: [
                    TextElement {
                        value: "Hello this is\n",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "a multiline\n",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "message.",
                    },
                ],
            },
        ),
        attributes: [],
        comment: Some(
            Comment {
                content: [
                    "Tabbing/spaces on the empty lines.",
                ],
            },
        ),
    },
)
Successfully found entry: Message(
    Message {
        id: Identifier {
            name: "multiline-message-args",
        },
        value: Some(
            Pattern {
                elements: [
                    TextElement {
                        value: "This is a multiline message.\n",
                    },
                    TextElement {
                        value: "\n",
                    },
                    Placeable {
                        expression: InlineExpression(
                            VariableReference {
                                id: Identifier {
                                    name: "argOne",
                                },
                            },
                        ),
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "This is a multiline message.\n",
                    },
                    TextElement {
                        value: "\n",
                    },
                    Placeable {
                        expression: InlineExpression(
                            VariableReference {
                                id: Identifier {
                                    name: "argTwo",
                                },
                            },
                        ),
                    },
                ],
            },
        ),
        attributes: [],
        comment: Some(
            Comment {
                content: [
                    "No spaces/tabbing on the empty lines.",
                ],
            },
        ),
    },
)
```

To reproduce the issue, ensure that the `test.ftl` is saved as CRLF. On unix, by default it is LF.

The expected output when the error occurs is the following:

```txt
Contents of test.ftl:

multiline-message = Hello this is
    a multiline message
    using fluent.
multiline-message-break =
    Hello this is
    a multiline
    message.
# No spaces/tabbing on the empty lines.
multiline-message-breaks =
    Hello this is

    a multiline

    message.
# Tabbing/spaces on the empty lines.
multiline-message-breaks-spaces =
    Hello this is
    
    a multiline
    
    message.
# No spaces/tabbing on the empty lines.
multiline-message-args =
    This is a multiline message.

    { $argOne }

    This is a multiline message.

    { $argTwo }

The following errors occurred while parsing test.ftl:
[ParserError { pos: (246, 247), slice: Some((246, 279)), kind: ExpectedCharRange { range: "a-zA-Z" } }, ParserError { pos: (517, 518), slice: Some((517, 587)), kind: ExpectedCharRange { range: "a-zA-Z" } }]
Successfully found entry: Message(
    Message {
        id: Identifier {
            name: "multiline-message",
        },
        value: Some(
            Pattern {
                elements: [
                    TextElement {
                        value: "Hello this is",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "a multiline message",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "using fluent.",
                    },
                ],
            },
        ),
        attributes: [],
        comment: None,
    },
)
Successfully found entry: Message(
    Message {
        id: Identifier {
            name: "multiline-message-break",
        },
        value: Some(
            Pattern {
                elements: [
                    TextElement {
                        value: "Hello this is",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "a multiline",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "message.",
                    },
                ],
            },
        ),
        attributes: [],
        comment: None,
    },
)
Successfully found entry: Message(
    Message {
        id: Identifier {
            name: "multiline-message-breaks",
        },
        value: Some(
            Pattern {
                elements: [
                    TextElement {
                        value: "Hello this is",
                    },
                ],
            },
        ),
        attributes: [],
        comment: Some(
            Comment {
                content: [
                    "No spaces/tabbing on the empty lines.",
                ],
            },
        ),
    },
)
Successfully found entry: Message(
    Message {
        id: Identifier {
            name: "multiline-message-breaks-spaces",
        },
        value: Some(
            Pattern {
                elements: [
                    TextElement {
                        value: "Hello this is",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "a multiline",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "message.",
                    },
                ],
            },
        ),
        attributes: [],
        comment: Some(
            Comment {
                content: [
                    "Tabbing/spaces on the empty lines.",
                ],
            },
        ),
    },
)
Successfully found entry: Message(
    Message {
        id: Identifier {
            name: "multiline-message-args",
        },
        value: Some(
            Pattern {
                elements: [
                    TextElement {
                        value: "This is a multiline message.",
                    },
                ],
            },
        ),
        attributes: [],
        comment: Some(
            Comment {
                content: [
                    "No spaces/tabbing on the empty lines.",
                ],
            },
        ),
    },
)
```

As you can see, it managed to find all the entries, except the `multiline-message-selector`, which had earlier triggered a `ParseError`.
