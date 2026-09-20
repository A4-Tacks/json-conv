use expect_test::expect;

use super::*;
use test_buffer::TestBuffer;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Obj {
    x: i8,
    y: Option<i8>,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Test {
    u32: u32,
    i32: i32,
    f32: f32,
    f64: f64,
    vec: [i8; 2],
    str: Vec<String>,
    obj: Obj,
}
fn data() -> &'static [Test] {
    static DATA: std::sync::LazyLock<Vec<Test>> = std::sync::LazyLock::new(|| vec![
        Test {
            u32: 2,
            i32: -3,
            f32: 2.5,
            f64: -1.2,
            vec: [1, -2],
            str: vec!["1 2 a".into(), ",".into()],
            obj: Obj { x: 3, y: None },
        },
        Test {
            u32: 2,
            i32: 0,
            f32: -f32::INFINITY,
            f64: f64::NAN,
            vec: [0, 100],
            str: vec!["1".into(), "NaN".into(), "null".into(), "\"`':-".into(), "\\{([])}".into()],
            obj: Obj { x: 3, y: Some(7) },
        },
    ]);
    &DATA
}
fn debug(s: &impl std::fmt::Debug) -> String {
    format!("{s:#?}")
}
fn diff(a: &str, b: &str) -> String {
    let diffs = diff::lines(a, b);
    if diffs.iter().all(|it| matches!(it, diff::Result::Both(..))) {
        return String::new();
    }
    diffs.iter().map(|x| match x {
        diff::Result::Left(s) => format!("-{s}"),
        diff::Result::Both(s, _) => format!(" {s}"),
        diff::Result::Right(s) => format!("+{s}"),
    }).collect::<Vec<_>>().join("\n")
}

#[test]
fn test_rebuild_pure() {
    let mut out = TestBuffer::new();
    let testdata = &data()[0];
    for &spec in Spec::all() {
        writeln!(out, "===> {spec:?} <===");
        let compact = testdata.to_specjson(spec).unwrap();
        let pretty = testdata.to_specjson_prettify(spec).unwrap();
        let from_compact = match Test::from_specjson(spec, &compact) {
            Ok(from_compact) => Some(from_compact),
            Err(e) => {
                writeln!(out, "ERROR: cannot rebuild: {e}");
                None
            }
        };
        let from_pretty = match Test::from_specjson(spec, &pretty) {
            Ok(it) => Some(it),
            Err(e) => {
                writeln!(out, "ERROR: cannot rebuild: {e}");
                None
            }
        };
        if debug(&from_compact) == debug(&from_pretty) {
            if &debug(&Some(&testdata)) != &debug(&from_pretty) {
                writeln!(out, "无法完美反序列化:");
            }
            writeln!(out, "{}", diff(&debug(&Some(&testdata)), &debug(&from_pretty)));
        } else {
            writeln!(out, "{}", diff(&debug(&from_compact), &debug(&from_pretty)));
        }
    }
    expect![[r#"
        ===> Json <===

        ===> Json5 <===

        ===> HJson <===

    "#]].assert_eq(&out);
}

#[test]
fn test_rebuild_special() {
    let mut out = TestBuffer::new();
    let testdata = &data()[1..];
    for &spec in Spec::all() {
        writeln!(out, "===> {spec:?} <===");
        let compact = testdata.to_specjson(spec).unwrap();
        let pretty = testdata.to_specjson_prettify(spec).unwrap();
        let from_compact = match Vec::<Test>::from_specjson(spec, &compact) {
            Ok(it) => Some(it),
            Err(e) => {
                writeln!(out, "ERROR: cannot rebuild: {e}");
                None
            }
        };
        let from_pretty = match Vec::<Test>::from_specjson(spec, &pretty) {
            Ok(it) => Some(it),
            Err(e) => {
                writeln!(out, "ERROR: cannot rebuild: {e}");
                None
            }
        };
        if debug(&from_compact) == debug(&from_pretty) {
            if &debug(&Some(&testdata)) != &debug(&from_pretty) {
                writeln!(out, "无法完美反序列化:");
            }
            writeln!(out, "{}", diff(&debug(&Some(&testdata)), &debug(&from_pretty)));
        } else {
            writeln!(out, "{}", diff(&debug(&from_compact), &debug(&from_pretty)));
        }
    }
    expect![[r#"
        ===> Json <===
        ERROR: cannot rebuild: invalid type: null, expected f32 at line 1 column 28
        ERROR: cannot rebuild: invalid type: null, expected f32 at line 5 column 15
        无法完美反序列化:
        -Some(
        -    [
        -        Test {
        -            u32: 2,
        -            i32: 0,
        -            f32: -inf,
        -            f64: NaN,
        -            vec: [
        -                0,
        -                100,
        -            ],
        -            str: [
        -                "1",
        -                "NaN",
        -                "null",
        -                "\"`':-",
        -                "\\{([])}",
        -            ],
        -            obj: Obj {
        -                x: 3,
        -                y: Some(
        -                    7,
        -                ),
        -            },
        -        },
        -    ],
        -)
        +None
        ===> Json5 <===

        ===> HJson <===
        ERROR: cannot rebuild: invalid type: unit value, expected f32 at line 0 column 0
        ERROR: cannot rebuild: invalid type: unit value, expected f32 at line 0 column 0
        无法完美反序列化:
        -Some(
        -    [
        -        Test {
        -            u32: 2,
        -            i32: 0,
        -            f32: -inf,
        -            f64: NaN,
        -            vec: [
        -                0,
        -                100,
        -            ],
        -            str: [
        -                "1",
        -                "NaN",
        -                "null",
        -                "\"`':-",
        -                "\\{([])}",
        -            ],
        -            obj: Obj {
        -                x: 3,
        -                y: Some(
        -                    7,
        -                ),
        -            },
        -        },
        -    ],
        -)
        +None
    "#]].assert_eq(&out);
}

#[test]
fn test_outputs() {
    let mut out = TestBuffer::new();
    for &spec in Spec::all() {
        writeln!(out, "===> {spec:?} <===");
        let compact = data().to_specjson(spec).unwrap();
        let pretty = data().to_specjson_prettify(spec).unwrap();
        writeln!(out, "{compact}\n{pretty}");
    }
    expect![[r#"
        ===> Json <===
        [{"u32":2,"i32":-3,"f32":2.5,"f64":-1.2,"vec":[1,-2],"str":["1 2 a",","],"obj":{"x":3,"y":null}},{"u32":2,"i32":0,"f32":null,"f64":null,"vec":[0,100],"str":["1","NaN","null","\"`':-","\\{([])}"],"obj":{"x":3,"y":7}}]
        [
          {
            "u32": 2,
            "i32": -3,
            "f32": 2.5,
            "f64": -1.2,
            "vec": [
              1,
              -2
            ],
            "str": [
              "1 2 a",
              ","
            ],
            "obj": {
              "x": 3,
              "y": null
            }
          },
          {
            "u32": 2,
            "i32": 0,
            "f32": null,
            "f64": null,
            "vec": [
              0,
              100
            ],
            "str": [
              "1",
              "NaN",
              "null",
              "\"`':-",
              "\\{([])}"
            ],
            "obj": {
              "x": 3,
              "y": 7
            }
          }
        ]
        ===> Json5 <===
        [{"u32":2,"i32":-3,"f32":2.5,"f64":-1.2,"vec":[1,-2],"str":["1 2 a",","],"obj":{"x":3,"y":null}},{"u32":2,"i32":0,"f32":-Infinity,"f64":NaN,"vec":[0,100],"str":["1","NaN","null","\"`':-","\\{([])}"],"obj":{"x":3,"y":7}}]
        [
          {
            u32: 2,
            i32: -3,
            f32: 2.5,
            f64: -1.2,
            vec: [
              1,
              -2,
            ],
            str: [
              "1 2 a",
              ",",
            ],
            obj: {
              x: 3,
              y: null,
            },
          },
          {
            u32: 2,
            i32: 0,
            f32: -Infinity,
            f64: NaN,
            vec: [
              0,
              100,
            ],
            str: [
              "1",
              "NaN",
              "null",
              "\"`':-",
              "\\{([])}",
            ],
            obj: {
              x: 3,
              y: 7,
            },
          },
        ]
        ===> HJson <===
        [{u32:2
        i32:-3
        f32:2.5
        f64:-1.2
        vec:[1
        -2]
        str:[1 2 a
        ","]
        obj:{x:3
        y:null}}
        {u32:2
        i32:0
        f32:null
        f64:null
        vec:[0
        100]
        str:["1"
        NaN
        "null"
        '''"`':-'''
        \{([])}]
        obj:{x:3
        y:7}}]
        [
          {
            u32: 2
            i32: -3
            f32: 2.5
            f64: -1.2
            vec:
            [
              1
              -2
            ]
            str:
            [
              1 2 a
              ","
            ]
            obj:
            {
              x: 3
              y: null
            }
          }
          {
            u32: 2
            i32: 0
            f32: null
            f64: null
            vec:
            [
              0
              100
            ]
            str:
            [
              "1"
              NaN
              "null"
              '''"`':-'''
              \{([])}
            ]
            obj:
            {
              x: 3
              y: 7
            }
          }
        ]
    "#]].assert_eq(&out);
}
