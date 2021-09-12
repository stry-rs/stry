use std::{collections::HashMap, fmt::Display, io::Write};

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Key<'p> {
    Segment(&'p str),
    Parameter(&'p str),
}

impl<'p> Key<'p> {
    fn from(p: &'p str) -> Self {
        match p {
            part if part.starts_with(':') => Key::Parameter(part.trim_start_matches(':')),
            part => Key::Segment(part),
        }
    }
}

#[derive(Debug)]
enum Value<'p, T> {
    Map(Option<T>, HashMap<Key<'p>, Value<'p, T>>),
    End(T),
}

impl<'p, T> Value<'p, T> {
    fn map() -> Self {
        Value::Map(None, HashMap::new())
    }
}

impl<'p, T> Default for Value<'p, T> {
    fn default() -> Self {
        Value::map()
    }
}

fn add<'p, I, T>(
    longest: &mut usize,
    mut current: usize,
    parts: &mut std::iter::Peekable<I>,
    map: &mut HashMap<Key<'p>, Value<'p, T>>,
    data: T,
) where
    I: Iterator<Item = Key<'p>>,
    T: std::fmt::Debug,
{
    if let Some(part) = parts.next() {
        if matches!(part, Key::Parameter(_)) {
            current += 1;

            if current > *longest {
                *longest = current;
            }
        }

        match map.get(&part) {
            Some(Value::Map(_, _)) => {
                if parts.peek().is_some() {
                    if let Value::Map(_data, map) = map.get_mut(&part).unwrap() {
                        add(longest, current, parts, map, data);
                    }
                } else if let Value::Map(map_data, _map) = map.get_mut(&part).unwrap() {
                    if map_data.is_none() {
                        *map_data = Some(data);
                    }
                }
            }
            Some(Value::End(_)) => {
                if parts.peek().is_some() {
                    {
                        let value = map.remove(&part).unwrap();

                        let data = match value {
                            Value::Map(data, _) => data,
                            Value::End(data) => Some(data),
                        };

                        map.insert(part, Value::Map(data, HashMap::new()));
                    }

                    if let Value::Map(_data, map) = map.get_mut(&part).unwrap() {
                        add(longest, current, parts, map, data);
                    }
                }
            }
            None => {
                if parts.peek().is_some() {
                    if let Value::Map(_data, map) = map.entry(part).or_insert_with(Value::map) {
                        add(longest, current, parts, map, data);
                    }
                } else {
                    map.entry(part).or_insert_with(|| Value::End(data));
                }
            }
        }
    }
}

fn writer<'p, W, T>(
    w: &mut W,
    map: &HashMap<Key<'p>, Value<'p, T>>,
    data: Option<&T>,
) -> std::io::Result<()>
where
    W: Write,
    T: Display,
{
    writeln!(w, "    match parts.next() {{")?;

    let mut entries = map.iter().collect::<Vec<_>>();
    entries.sort_by_key(|(k, _)| *k);

    for (key, value) in entries {
        match key {
            Key::Segment(text) => writeln!(w, "        Some({:?}) => {{", text)?,
            Key::Parameter(ident) => {
                writeln!(w, "        Some(r#{}) => {{", ident)?;
                writeln!(w, "            params.insert({:?}, r#{});", ident, ident)?;
            }
        }

        match value {
            Value::Map(data, map) => {
                writer(w, map, data.as_ref())?;
            }
            Value::End(data) => {
                writeln!(w, "Some((params, Box::new(|d, r, p| {}(d, r, p))))", data)?
            }
        }

        writeln!(w, "}}")?;
    }

    if let Some(data) = data {
        writeln!(
            w,
            "        None => Some((params, Box::new(|d, r, p| {}(d, r, p)))),",
            data
        )?
    }

    writeln!(w, "        _ => None,")?;

    writeln!(w, "    }}")?;

    Ok(())
}

pub fn build<'p, W, T>(
    w: &mut W,
    routes: HashMap<&'p str, Vec<(&'p str, T)>>,
) -> std::io::Result<()>
where
    W: Write,
    T: Display + Copy + std::fmt::Debug,
{
    let mut longest = 0;
    let mut map: HashMap<&'p str, HashMap<Key<'p>, Value<'p, T>>> = HashMap::new();

    for (method, value) in &routes {
        for (route, data) in value {
            let mut parts = route
                .split('/')
                .filter(|p| !p.is_empty())
                .map(Key::from)
                .peekable();

            add(
                &mut longest,
                0,
                &mut parts,
                map.entry(*method).or_insert_with(HashMap::new),
                *data,
            );
        }
    }

    writeln!(
        w,
        r#"pub struct Params<'u> {{
    index: usize,
    array: [::std::mem::MaybeUninit<(&'static str, &'u str)>; {}],
}}"#,
        longest
    )?;

    writeln!(
        w,
        r#"impl<'u> Params<'u> {{
    pub fn get<S: AsRef<str>>(&self, key: S) -> Option<&'u str> {{
        fn inner<'u>(params: &Params<'u>, key: &str) -> Option<&'u str> {{
            for entry in &params.array[0..params.index] {{
                let (k, v) = unsafe {{ entry.assume_init() }};
                if k == key {{
                    return Some(v);
                }}
            }}
            None
        }}
        inner(self, key.as_ref())
    }}
}}"#
    )?;
    writeln!(
        w,
        "pub struct Data<D> {{
    inner: ::std::sync::Arc<D>,
}}
impl<D> Data<D> {{
    pub fn new(data: D) -> Self {{
        Self {{
            inner: ::std::sync::Arc::new(data),
        }}
    }}
}}
impl<D> Clone for Data<D> {{
    fn clone(&self) -> Self {{
        Self {{
            inner: self.inner.clone(),
        }}
    }}
}}
impl<D> std::ops::Deref for Data<D> {{
    type Target = ::std::sync::Arc<D>;
    fn deref(&self) -> &Self::Target {{
        &self.inner
    }}
}}"
    )?;

    writeln!(w, "pub type Response = ::hyper::Response<::hyper::Body>;")?;
    writeln!(w, "pub type Request = ::hyper::Request<::hyper::Body>;")?;
    writeln!(w, "pub type Return<'u> = std::boxed::Box<dyn Fn(Data<::stry_common::backend::boxed::BoxedBackend>, Request, Params<'u>) -> ::std::pin::Pin<std::boxed::Box<dyn ::std::future::Future<Output = ::std::result::Result<Response, ::stry_common::prelude::Error>> + Send + 'u>> + Send + Sync + 'u>;")?;

    writeln!(
        w,
        "#[allow(dead_code, unreachable_patterns, unused_variables, clippy::manual_map)]"
    )?;
    writeln!(
        w,
        "pub fn find<'u>(method: &::hyper::Method, url: &'u str) -> Option<(Params<'u>, Return<'u>)> {{",
    )?;

    writeln!(
        w,
        r#"impl<'u> Params<'u> {{
    fn new() -> Self {{
        Self {{
            index: 0,
            array: unsafe {{
                ::std::mem::MaybeUninit::uninit().assume_init()
            }},
        }}
    }}

    fn insert(&mut self, key: &'static str, value: &'u str) {{
        {{
            let entry = &mut self.array[self.index];
            *entry = ::std::mem::MaybeUninit::new((key, value));
        }}
        self.index += 1;
    }}
}}"#
    )?;
    writeln!(
        w,
        "    let mut parts = url.split('/').filter(|p| !p.is_empty());"
    )?;
    writeln!(w, "    let mut params: Params<'u> = Params::new();")?;
    writeln!(w, "    match *method {{")?;
    for (method, map) in &map {
        writeln!(
            w,
            "        ::hyper::Method::{} => {{",
            method.to_uppercase()
        )?;
        writer(w, map, None)?;
        writeln!(w, "        }}")?;
    }
    writeln!(w, "        _ => None,")?;
    writeln!(w, "    }}")?;
    writeln!(w, "}}")?;

    Ok(())
}
