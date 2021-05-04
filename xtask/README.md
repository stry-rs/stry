# cargo xtask

The [cargo-xtask](https://github.com/matklad/cargo-xtask) for the stry monorepo.

## ArkDL

A data definition language that abstracts over GraphQL and ProtoBuf.

```
PageInfo :: type {
    start_cursor: String
    end_cursor: String
    has_next_page: Boolean
    has_previous_page: Boolean
}

Edge <T> :: type {
    node: T
    cursor: String
}

Connection <T> :: type {
    edges: [Edge <T>]
    page_info: PageInfo
}


Character :: type {
    name: String
}


Query :: service {
    characters :: fn (
        first: Int,
        last: Int,
        before: String,
        after: String,
    ) -> Connection <Character>
}

Mutation :: service {}
```
