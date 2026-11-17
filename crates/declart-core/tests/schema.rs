//! `schema` must agree with the parser in both directions: whatever it lists parses, whatever it
//! requires is required, every count and rule it states is enforced, and every field the parser reads
//! is listed.

use declart_core::{parse_json, schema, views, Collection, Field, FieldType, Schema, KINDS};
use serde_json::{json, Map, Value};
use std::collections::BTreeSet;

/// Every kind with every view, and the kinds that have views once more with none given.
fn cases() -> Vec<(&'static str, Option<&'static str>)> {
    let mut all = Vec::new();
    for kind in KINDS {
        let known = views(kind).expect("every kind lists its views");
        if known.is_empty() {
            all.push((*kind, None));
        } else {
            all.push((*kind, None));
            for view in known {
                all.push((*kind, Some(*view)));
            }
        }
    }
    all
}

fn labelled(prefix: &str, n: usize) -> Vec<Value> {
    (0..n)
        .map(|i| json!({ "label": format!("{prefix}{i}") }))
        .collect()
}

/// The smallest declaration that parses: the required fields, plus the one thing the rules ask for
/// that no field marks as required (a timeline event needs a `date`).
fn fixture(kind: &str, view: Option<&str>) -> Value {
    let mut v = match kind {
        "flow" => {
            let n = if matches!(view, Some("cycle" | "funnel" | "swimlane")) {
                2
            } else {
                1
            };
            let mut items = labelled("L", n);
            if view == Some("swimlane") {
                for (i, item) in items.iter_mut().enumerate() {
                    item["actor"] = json!(if i % 2 == 0 { "a" } else { "b" });
                }
            }
            json!({ "kind": kind, "items": items })
        }
        "tier" => json!({ "kind": kind, "items": labelled("L", 1) }),
        "hierarchy" => {
            let roots = if view == Some("fishbone") { 2 } else { 1 };
            json!({ "kind": kind, "nodes": labelled("L", roots) })
        }
        "timeline" => json!({ "kind": kind, "events": [
            { "date": "2024-01-01", "label": "L0" },
            { "date": "2024-02-01", "label": "L1" },
        ]}),
        "matrix" => {
            json!({ "kind": kind, "x_axis": "X", "y_axis": "Y", "quadrants": labelled("L", 4) })
        }
        "hub_spoke" => json!({ "kind": kind, "center": "C", "spokes": labelled("L", 2) }),
        "venn" => json!({ "kind": kind, "sets": labelled("L", 2) }),
        "comparison" => {
            json!({ "kind": kind, "columns": labelled("C", 1), "rows": labelled("R", 1) })
        }
        "state" => json!({ "kind": kind, "states": labelled("L", 2) }),
        other => panic!("no fixture for {other}"),
    };
    if let Some(view) = view {
        v["view"] = json!(view);
    }
    v
}

/// Keys a fixture carries that the schema does not mark required, because a rule asks for them.
fn rule_keys(kind: &str, collection: &str) -> &'static [&'static str] {
    match (kind, collection) {
        ("timeline", "events") => &["date"],
        _ => &[],
    }
}

fn sample(f: &Field) -> Value {
    match f.ty {
        FieldType::String | FieldType::Reference => json!("L0"),
        FieldType::Enum => json!(f.values[0]),
        FieldType::Date => json!("2024-01-01"),
        FieldType::StringList => json!(["L0", "L1"]),
    }
}

/// Makes sure the declaration has the collection, with one element holding its required fields.
fn ensure(v: &mut Value, c: &Collection) {
    if v.get(c.name).is_none() {
        let mut e = Map::new();
        for f in c.fields.iter().filter(|f| f.required) {
            e.insert(f.name.to_string(), sample(f));
        }
        v[c.name] = json!([e]);
    }
}

fn parses(v: &Value) -> Result<(), String> {
    parse_json(&v.to_string())
        .map(|_| ())
        .map_err(|e| e.to_string())
}

fn keys(v: &Value) -> BTreeSet<String> {
    v.as_object()
        .map(|o| o.keys().cloned().collect())
        .unwrap_or_default()
}

fn get(kind: &str, view: Option<&str>) -> Schema {
    schema(kind, view).unwrap_or_else(|e| panic!("{kind}/{view:?}: {e}"))
}

// ---- schema → parser --------------------------------------------------------------------------

#[test]
fn the_fixture_is_the_required_fields_and_nothing_else() {
    for (kind, view) in cases() {
        let s = get(kind, view);
        let v = fixture(kind, view);
        parses(&v).unwrap_or_else(|e| panic!("{kind}/{view:?} fixture: {e}"));

        let collection_names: BTreeSet<&str> = s.collections.iter().map(|c| c.name).collect();
        let top: BTreeSet<String> = keys(&v)
            .into_iter()
            .filter(|k| k != "view" && !collection_names.contains(k.as_str()))
            .collect();
        let required: BTreeSet<String> = s
            .fields
            .iter()
            .filter(|f| f.required)
            .map(|f| f.name.to_string())
            .collect();
        assert_eq!(top, required, "{kind}/{view:?} top-level fields");

        for c in &s.collections {
            let Some(first) = v.get(c.name).and_then(|a| a.get(0)) else {
                assert_eq!(c.min, 0, "{kind}/{view:?} {}: a fixture needs it", c.name);
                continue;
            };
            let mut required: BTreeSet<String> = c
                .fields
                .iter()
                .filter(|f| f.required)
                .map(|f| f.name.to_string())
                .collect();
            required.extend(rule_keys(kind, c.name).iter().map(|k| k.to_string()));
            assert_eq!(keys(first), required, "{kind}/{view:?} {} fields", c.name);
        }
    }
}

#[test]
fn a_required_field_is_required() {
    for (kind, view) in cases() {
        let s = get(kind, view);
        let v = fixture(kind, view);
        for f in s.fields.iter().filter(|f| f.required) {
            let mut broken = v.clone();
            broken.as_object_mut().unwrap().remove(f.name);
            assert!(
                parses(&broken).is_err(),
                "{kind}/{view:?}: `{}` is listed as required but a declaration without it parses",
                f.name
            );
        }
        for c in &s.collections {
            let mut v = v.clone();
            ensure(&mut v, c);
            for f in c.fields.iter().filter(|f| f.required) {
                let mut broken = v.clone();
                broken[c.name][0].as_object_mut().unwrap().remove(f.name);
                assert!(
                    parses(&broken).is_err(),
                    "{kind}/{view:?}: `{}[].{}` is listed as required but a declaration without it parses",
                    c.name,
                    f.name
                );
            }
        }
    }
}

/// Fields a generic "set it to a sample" test cannot judge on its own: they belong to a rule.
fn belongs_to_a_rule(kind: &str, collection: &str, field: &str) -> bool {
    matches!(
        (kind, collection, field),
        ("timeline", "events", "date" | "start" | "end")
            | ("hierarchy", "nodes", "parent")
            | ("comparison", "rows", "<column>")
            | ("matrix", "quadrants", "position")
    )
}

#[test]
fn an_optional_field_is_accepted_and_a_listed_value_is_too() {
    for (kind, view) in cases() {
        let s = get(kind, view);
        let base = fixture(kind, view);
        for c in &s.collections {
            let mut base = base.clone();
            ensure(&mut base, c);
            for f in c
                .fields
                .iter()
                .filter(|f| !f.required || f.ty == FieldType::Enum)
            {
                if belongs_to_a_rule(kind, c.name, f.name) {
                    continue;
                }
                let samples: Vec<Value> = match f.ty {
                    FieldType::Enum => f.values.iter().map(|v| json!(v)).collect(),
                    _ => vec![sample(f)],
                };
                for value in samples {
                    let mut v = base.clone();
                    v[c.name][0][f.name] = value.clone();
                    parses(&v).unwrap_or_else(|e| {
                        panic!("{kind}/{view:?} {}[].{} = {value}: {e}", c.name, f.name)
                    });
                }
                if f.ty == FieldType::Enum {
                    let mut v = base.clone();
                    v[c.name][0][f.name] = json!("not-a-listed-value");
                    assert!(
                        parses(&v).is_err(),
                        "{kind}/{view:?} {}[].{} takes any value",
                        c.name,
                        f.name
                    );
                }
            }
        }
        for f in s.fields.iter().filter(|f| !f.required) {
            if matches!(f.name, "view") {
                continue;
            }
            let samples: Vec<Value> = match f.ty {
                FieldType::Enum => f.values.iter().map(|v| json!(v)).collect(),
                _ => vec![json!("Sample")],
            };
            for value in samples {
                let mut v = base.clone();
                v[f.name] = value.clone();
                parses(&v).unwrap_or_else(|e| panic!("{kind}/{view:?} {} = {value}: {e}", f.name));
            }
        }
    }
}

#[test]
fn every_quadrant_position_is_accepted_when_all_four_are_given() {
    let s = get("matrix", None);
    let quadrants = s
        .collections
        .iter()
        .find(|c| c.name == "quadrants")
        .unwrap();
    let positions = &quadrants
        .fields
        .iter()
        .find(|f| f.name == "position")
        .unwrap()
        .values;
    assert_eq!(positions.len(), quadrants.min);
    for rotate in 0..positions.len() {
        let mut v = fixture("matrix", None);
        for i in 0..positions.len() {
            v["quadrants"][i]["position"] = json!(positions[(i + rotate) % positions.len()]);
        }
        parses(&v).unwrap_or_else(|e| panic!("positions rotated by {rotate}: {e}"));
    }
    // A position outside the list is refused.
    let mut v = fixture("matrix", None);
    for i in 0..positions.len() {
        v["quadrants"][i]["position"] = json!(positions[i]);
    }
    v["quadrants"][0]["position"] = json!("centre");
    assert!(parses(&v).is_err());
}

#[test]
fn a_view_is_accepted_when_listed_and_not_otherwise() {
    for kind in KINDS {
        let known = views(kind).unwrap();
        let s = get(kind, None);
        assert_eq!(s.views, known, "{kind}");
        if known.is_empty() {
            assert!(s.fields.iter().all(|f| f.name != "view"), "{kind}");
            continue;
        }
        let listed = s.fields.iter().find(|f| f.name == "view").unwrap();
        assert_eq!(listed.values, known, "{kind}");
        let mut v = fixture(kind, None);
        v["view"] = json!("not-a-view");
        assert!(parses(&v).is_err(), "{kind} takes any view");
        for view in known {
            parses(&fixture(kind, Some(view))).unwrap_or_else(|e| panic!("{kind}/{view}: {e}"));
        }
        // The default view is the one a declaration without `view` is read as.
        if let Some(default) = s.default_view {
            assert!(known.contains(&default), "{kind}");
            assert_eq!(
                get(kind, None).collections,
                get(kind, Some(default)).collections,
                "{kind}: no view reads as `{default}`"
            );
        }
    }
}

#[test]
fn a_date_that_is_not_a_date_is_refused() {
    for field in ["date", "start", "end"] {
        for bad in ["soon", "2024-13-01", "2024-02-30"] {
            let mut v = fixture("timeline", None);
            v["events"][0] = if field == "date" {
                json!({ "date": bad, "label": "L0" })
            } else {
                json!({ "start": "2024-01-01", "end": "2024-02-01", field: bad, "label": "L0" })
            };
            assert!(parses(&v).is_err(), "{field} = {bad}");
        }
    }
}

// ---- counts -----------------------------------------------------------------------------------

/// The collection with `n` elements, each a copy of its first with a distinct label.
fn with_count(v: &Value, c: &Collection, n: usize) -> Value {
    let mut v = v.clone();
    ensure(&mut v, c);
    let first = v[c.name][0].clone();
    let elements: Vec<Value> = (0..n)
        .map(|i| {
            let mut e = first.clone();
            if e.get("label").is_some() {
                e["label"] = json!(format!("L{i}"));
            }
            if e.get("actor").is_some() {
                e["actor"] = json!(if i % 2 == 0 { "a" } else { "b" });
            }
            e
        })
        .collect();
    v[c.name] = Value::Array(elements);
    v
}

#[test]
fn a_collection_takes_exactly_the_counts_it_lists() {
    for (kind, view) in cases() {
        // A hierarchy is counted by its roots, which is a rule below.
        if kind == "hierarchy" {
            continue;
        }
        let s = get(kind, view);
        let base = fixture(kind, view);
        for c in &s.collections {
            let at = |n: usize| parses(&with_count(&base, c, n));
            at(c.min).unwrap_or_else(|e| {
                panic!("{kind}/{view:?} {} at its minimum {}: {e}", c.name, c.min)
            });
            if c.min > 0 {
                assert!(
                    at(c.min - 1).is_err(),
                    "{kind}/{view:?}: {} takes fewer than its minimum {}",
                    c.name,
                    c.min
                );
            }
            if let Some(max) = c.max {
                at(max).unwrap_or_else(|e| {
                    panic!("{kind}/{view:?} {} at its maximum {max}: {e}", c.name)
                });
                assert!(
                    at(max + 1).is_err(),
                    "{kind}/{view:?}: {} takes more than its maximum {max}",
                    c.name
                );
            }
        }
    }
}

// ---- rules ------------------------------------------------------------------------------------

type Mutation = fn(&mut Value);

fn m(f: Mutation) -> Mutation {
    f
}

fn push(v: &mut Value, collection: &str, element: Value) {
    v[collection].as_array_mut().unwrap().push(element);
}

/// For each rule of the schema, in order, declarations that break it. Each must fail to parse.
fn violations(kind: &str, view: Option<&str>) -> Vec<Vec<Mutation>> {
    match (kind, view) {
        ("flow", Some("swimlane")) => vec![
            vec![m(|v| v["items"][0]["actor"] = json!(""))],
            vec![m(|v| v["items"][1]["actor"] = json!("a"))],
        ],
        ("flow", _) | ("tier", _) | ("hub_spoke", _) | ("venn", _) => vec![],
        ("hierarchy", view) => {
            let roots: Vec<Mutation> = match view {
                Some("fishbone") => vec![
                    m(|v| v["nodes"] = json!([{ "label": "L0" }])),
                    m(|v| v["nodes"] = Value::Array(labelled("L", 21))),
                ],
                None => vec![m(|v| v["nodes"] = Value::Array(labelled("L", 21)))],
                _ => vec![m(|v| push(v, "nodes", json!({ "label": "Y" })))],
            };
            vec![
                vec![m(|v| {
                    push(v, "nodes", json!({ "label": "L0", "parent": "L0" }))
                })],
                vec![m(|v| {
                    v["nodes"][0]["id"] = json!("x");
                    push(v, "nodes", json!({ "label": "Y", "id": "x" }));
                })],
                vec![
                    m(|v| push(v, "nodes", json!({ "label": "Y", "parent": "missing" }))),
                    m(|v| push(v, "nodes", json!({ "label": "Y", "parent": "Y" }))),
                ],
                roots,
            ]
        }
        ("timeline", _) => vec![
            vec![
                m(|v| v["events"][0]["start"] = json!("2024-01-01")),
                m(|v| {
                    v["events"][0].as_object_mut().unwrap().remove("date");
                }),
                m(|v| {
                    v["events"][0].as_object_mut().unwrap().remove("date");
                    v["events"][0]["start"] = json!("2024-01-01");
                }),
            ],
            vec![m(
                |v| {
                    v["events"][0] =
                        json!({ "start": "2024-05-01", "end": "2024-01-01", "label": "L0" })
                },
            )],
        ],
        ("matrix", _) => vec![
            vec![m(|v| v["quadrants"][0]["position"] = json!("top-left"))],
            vec![m(|v| {
                v["items"] = Value::Array(
                    (0..7)
                        .map(|i| json!({ "label": format!("I{i}"), "quadrant": "top-left" }))
                        .collect(),
                )
            })],
        ],
        ("comparison", _) => vec![
            vec![m(|v| push(v, "columns", json!({ "label": "label" })))],
            vec![m(|v| v["rows"][0]["nope"] = json!("x"))],
        ],
        ("state", _) => vec![
            vec![m(|v| push(v, "states", json!({ "label": "L0" })))],
            vec![m(|v| {
                v["states"][0]["id"] = json!("x");
                v["states"][1]["id"] = json!("x");
            })],
            vec![m(|v| {
                v["states"][0]["role"] = json!("initial");
                v["states"][1]["role"] = json!("initial");
            })],
            vec![m(|v| {
                v["transitions"] = json!([{ "from": "ghost", "to": "L1" }])
            })],
        ],
        other => panic!("no rule cases for {other:?}"),
    }
}

#[test]
fn every_rule_is_enforced() {
    for (kind, view) in cases() {
        let s = get(kind, view);
        let cases = violations(kind, view);
        assert_eq!(
            cases.len(),
            s.rules.len(),
            "{kind}/{view:?}: a rule without a test, or a test without a rule — {:?}",
            s.rules
        );
        for (rule, mutations) in s.rules.iter().zip(cases) {
            assert!(
                !mutations.is_empty(),
                "{kind}/{view:?}: `{rule}` has no case"
            );
            for mutate in mutations {
                let mut v = fixture(kind, view);
                mutate(&mut v);
                assert!(
                    parses(&v).is_err(),
                    "{kind}/{view:?}: a declaration that breaks `{rule}` parses: {v}"
                );
            }
        }
    }
}

#[test]
fn a_hierarchy_without_a_view_is_an_org_chart_or_a_fishbone() {
    let s = get("hierarchy", None);
    assert_eq!(s.view, None);
    assert_eq!(s.default_view, None, "the choice depends on the roots");
    parses(&fixture("hierarchy", None)).unwrap();
    let mut two = fixture("hierarchy", None);
    two["nodes"] = Value::Array(labelled("L", 2));
    parses(&two).unwrap();
}

// ---- parser → schema --------------------------------------------------------------------------

/// The names serde lists when a declaration holds a field it does not know — the parser's own list.
fn fields_the_parser_reads(v: &Value, at: Option<(&str, usize)>) -> BTreeSet<String> {
    let mut v = v.clone();
    match at {
        None => v["__probe__"] = json!("x"),
        Some((collection, i)) => v[collection][i]["__probe__"] = json!("x"),
    }
    let message = parse_json(&v.to_string()).unwrap_err().to_string();
    let listed = message
        .split_once("expected")
        .unwrap_or_else(|| panic!("no list of fields in: {message}"))
        .1;
    listed
        .split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect()
}

#[test]
fn every_field_the_parser_reads_is_listed() {
    for kind in KINDS {
        let known = views(kind).unwrap();
        let mut schemas = vec![get(kind, None)];
        schemas.extend(known.iter().map(|v| get(kind, Some(v))));

        // Fields at the top: the parser's list also holds the collections.
        let mut listed: BTreeSet<String> = BTreeSet::new();
        for s in &schemas {
            listed.extend(s.fields.iter().map(|f| f.name.to_string()));
            listed.extend(s.collections.iter().map(|c| c.name.to_string()));
        }
        let read = fields_the_parser_reads(&fixture(kind, None), None);
        assert_eq!(read, listed, "{kind}: fields at the top");

        // Fields of each collection.
        let mut by_collection: Vec<(&str, BTreeSet<String>)> = Vec::new();
        for s in &schemas {
            for c in &s.collections {
                match by_collection.iter_mut().find(|(n, _)| *n == c.name) {
                    Some((_, fields)) => fields.extend(c.fields.iter().map(|f| f.name.to_string())),
                    None => by_collection.push((
                        c.name,
                        c.fields.iter().map(|f| f.name.to_string()).collect(),
                    )),
                }
            }
        }
        for (name, listed) in by_collection {
            let mut v = fixture(kind, None);
            let c = get(kind, None)
                .collections
                .into_iter()
                .find(|c| c.name == name)
                .unwrap();
            ensure(&mut v, &c);
            if (*kind, name) == ("comparison", "rows") {
                // A row takes any key as a cell, so the parser has no list: it refuses a key that
                // is not a column instead.
                v["rows"][0]["__probe__"] = json!("x");
                let e = parse_json(&v.to_string()).unwrap_err().to_string();
                assert!(e.contains("not a declared column"), "{e}");
                assert_eq!(
                    listed,
                    BTreeSet::from(["label".to_string(), "<column>".to_string()])
                );
                continue;
            }
            let read = fields_the_parser_reads(&v, Some((name, 0)));
            assert_eq!(read, listed, "{kind}: fields of {name}");
        }
    }
}

#[test]
fn an_unknown_kind_or_view_is_an_error_with_a_hint() {
    assert!(schema("nope", None)
        .unwrap_err()
        .to_string()
        .contains("flow"));
    let e = schema("flow", Some("nope")).unwrap_err().to_string();
    assert!(e.contains("process") && e.contains("swimlane"), "{e}");
    let e = schema("timeline", Some("nope")).unwrap_err().to_string();
    assert!(e.contains("no views"), "{e}");
}

#[test]
fn the_schema_serializes_to_the_documented_shape() {
    let v = serde_json::to_value(get("flow", Some("swimlane"))).unwrap();
    assert_eq!(v["kind"], "flow");
    assert_eq!(v["view"], "swimlane");
    assert_eq!(v["default_view"], "process");
    assert_eq!(v["collections"][0]["name"], "items");
    assert_eq!(v["collections"][0]["min"], 2);
    assert_eq!(v["collections"][0]["max"], Value::Null);
    let actor = v["collections"][0]["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == "actor")
        .unwrap();
    assert_eq!(actor["type"], "string");
    assert_eq!(actor["required"], true);
    let status = v["collections"][0]["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == "status")
        .unwrap();
    assert_eq!(status["type"], "enum");
    assert_eq!(status["values"][0], "success");
    assert!(status.get("note").is_none());
}
