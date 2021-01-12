# Minimal Example for [Fluent Issue #213](https://github.com/projectfluent/fluent-rs/issues/213)

This is a minimal reproduction of the issue <https://github.com/projectfluent/fluent-rs/issues/213>.

The expected successful output of the program is the following:

```txt
Contents of test.ftl:

multiline-message-indented-breaks = 
    This is a multiline message 1.
    
    This is a multiline message 2.
multiline-message = 
    This is a multiline message 1.

    This is a multiline message 2.

Successfully found entry: Message(
    Message {
        id: Identifier {
            name: "multiline-message-indented-breaks",
        },
        value: Some(
            Pattern {
                elements: [
                    TextElement {
                        value: "This is a multiline message 1.\n",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "This is a multiline message 2.",
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
            name: "multiline-message",
        },
        value: Some(
            Pattern {
                elements: [
                    TextElement {
                        value: "This is a multiline message 1.\n",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "This is a multiline message 2.",
                    },
                ],
            },
        ),
        attributes: [],
        comment: None,
    },
)
```

To reproduce the issue, ensure that the `test.ftl` is saved as CRLF. On unix, by default it is LF.

The expected output when the error occurs is the following:

```txt
Contents of test.ftl:

multiline-message-indented-breaks = 
    This is a multiline message 1.
    
    This is a multiline message 2.
multiline-message = 
    This is a multiline message 1.

    This is a multiline message 2.

The following errors occurred while parsing test.ftl:
[ParserError { pos: (176, 177), slice: Some((176, 210)), kind: ExpectedCharRange { range: "a-zA-Z" } }]
Successfully found entry: Message(
    Message {
        id: Identifier {
            name: "multiline-message-indented-breaks",
        },
        value: Some(
            Pattern {
                elements: [
                    TextElement {
                        value: "This is a multiline message 1.",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "\n",
                    },
                    TextElement {
                        value: "This is a multiline message 2.",
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
            name: "multiline-message",
        },
        value: Some(
            Pattern {
                elements: [
                    TextElement {
                        value: "This is a multiline message 1.",
                    },
                ],
            },
        ),
        attributes: [],
        comment: None,
    },
)
```

As you can see, it managed to find all the entries, however there was a parse error and `multiline-message` is missing subsequent lines.
