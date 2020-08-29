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