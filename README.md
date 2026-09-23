CLI tool, convert json, json5, hjson

# Examples

```sh
$ json-conv <<< '{"a":[2, "foo"]}'
{
  "a": [
    2,
    "foo"
  ]
}
$ json-conv -t json5 <<< '{"a":[2, "foo"]}'
{
  a: [
    2,
    "foo",
  ],
}
$ json-conv -t hjson <<< '{"a":[2, "foo"]}'
{
  a:
  [
    2
    foo
  ]
}
$ json-conv <<< '{x:3}'
{
  "x": 3
}
```
