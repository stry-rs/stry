mod ark;

use {
    chrono::Utc,
    std::{
        collections::HashMap,
        fs::{self, File},
        io::Write as _,
        path::PathBuf,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("codegen") => match args.next().as_deref() {
            Some("device-detector") => device_detector()?,
            Some("proto") => {}
            None => {}
            Some(_) => {}
        },
        Some("new") => match args.next().as_deref() {
            Some("migration") => {
                let name = args
                    .next()
                    .expect("there must be a <NAME> argument to `xtask new migration`");
                let folder = args.next().unwrap_or_else(|| "./migrations".to_string());

                new_migration(&name, &folder)?;
            }
            None => {}
            Some(_) => {}
        },
        Some("--help") | None => {
            eprintln!("Tasks:");
            eprintln!();
            eprintln!("codegen");
            eprintln!("    proto        <FILE>");
            eprintln!("new");
            eprintln!("    migration    <NAME> <FOLDER>");
        }
        Some(cmd) => {
            eprintln!("unknown command: `{}`", cmd);
        }
    }

    Ok(())
}

fn device_detector() -> Result<(), Box<dyn std::error::Error>> {
    macro_rules! urls {
        ($repo:expr, $hash:expr, [ $( $file:expr , )* ]) => {
            [
                $(
                    (concat!($repo, "/", $hash, "/", $file), $file),
                )*
            ]
        };
    }

    static FILE_URLS: [(&str, &str); 16] = urls! {
        "https://raw.githubusercontent.com/matomo-org/device-detector",
        "29e21b2e745e09dedef82f5c4b2b9f8370a40368",
        [
            "regexes/bots.yml",
            "regexes/oss.yml",
            "regexes/client/browser_engine.yml",
            "regexes/client/browsers.yml",
            "regexes/client/feed_readers.yml",
            "regexes/client/libraries.yml",
            "regexes/client/mediaplayers.yml",
            "regexes/client/mobile_apps.yml",
            "regexes/client/pim.yml",
            "regexes/device/cameras.yml",
            "regexes/device/car_browsers.yml",
            "regexes/device/consoles.yml",
            "regexes/device/mobiles.yml",
            "regexes/device/notebooks.yml",
            "regexes/device/portable_media_player.yml",
            "regexes/device/televisions.yml",
        ]
    };

    #[derive(Debug, serde::Deserialize)]
    #[serde(untagged)]
    pub enum Data {
        List(Vec<Element>),
        Map(HashMap<String, Brand>),
    }

    #[derive(Debug, serde::Deserialize)]
    pub struct Element {
        pub regex: String,
        pub name: String,
        pub version: Option<String>,
        pub category: Option<String>,
        pub url: Option<String>,
        pub producer: Option<Producer>,
    }

    #[derive(Debug, serde::Deserialize)]
    pub struct Producer {
        pub name: String,
        pub url: String,
    }

    #[derive(Debug, serde::Deserialize)]
    pub struct Brand {
        pub regex: String,
        pub device: Option<String>,
        pub model: Option<String>,
        pub models: Option<Vec<Model>>,
    }

    #[derive(Debug, serde::Deserialize)]
    pub struct Model {
        pub regex: String,
        pub model: String,
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let out_dir = manifest_dir.join(".cache").join("matomo");

    fs::create_dir_all(&out_dir)?;

    println!("Downloading Matomo data files");

    for (url, file_name) in &FILE_URLS {
        let out_file = out_dir.join(file_name);

        fs::create_dir_all(out_file.parent().unwrap())?;

        if out_file.exists() {
            println!("`{}` exists, skipping", file_name);

            continue;
        }

        let res = ureq::get(url).call()?;

        let body = res.into_string()?;

        let mut file = File::create(&out_file)?;

        write!(&mut file, "{}", body)?;
    }

    println!("Generation Rust data file");

    let mut out_data = Vec::new();

    writeln!(
        &mut out_data,
        "{}",
        quote::quote! {
            #[derive(Debug)]
            pub struct Element {
                pub regex: &'static str,
                pub name: &'static str,
                pub version: Option<&'static str>,
                pub category: Option<&'static str>,
                pub url: Option<&'static str>,
                pub producer: Option<Producer>,
            }

            #[derive(Debug)]
            pub struct Producer {
                pub name: &'static str,
                pub url: &'static str,
            }

            #[derive(Debug)]
            pub struct Brand {
                pub regex: &'static str,
                pub device: Option<&'static str>,
                pub model: Option<&'static str>,
                pub models: Option<&'static [Model]>,
            }

            #[derive(Debug)]
            pub struct Model {
                pub regex: &'static str,
                pub model: &'static str,
            }
        }
    )?;

    for (_, file_name) in &FILE_URLS {
        let upper = PathBuf::from(file_name)
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap()
            .replace(".yml", "")
            .to_uppercase();
        let upper = quote::format_ident!("{}", upper);

        let out_file = out_dir.join(file_name);

        let data_file = fs::read_to_string(&out_file)?;

        let de = serde_yaml::Deserializer::from_str(&data_file);

        match serde_path_to_error::deserialize(de) {
            Ok(Data::List(list)) => {
                let count = list.len();

                let data_iter = list.into_iter().map(|element| {
                    let Element {
                        regex,
                        name,
                        version,
                        category,
                        url,
                        producer,
                    } = element;

                    let version = if let Some(version) = version {
                        quote::quote! { Some(#version) }
                    } else {
                        quote::quote! { None }
                    };

                    let category = if let Some(category) = category {
                        quote::quote! { Some(#category) }
                    } else {
                        quote::quote! { None }
                    };

                    let url = if let Some(url) = url {
                        quote::quote! { Some(#url) }
                    } else {
                        quote::quote! { None }
                    };

                    let producer = if let Some(producer) = producer {
                        let Producer { name, url } = producer;

                        quote::quote! { Some(Producer {
                            name: #name,
                            url: #url,
                        }) }
                    } else {
                        quote::quote! { None }
                    };

                    quote::quote! {
                        Element {
                            regex: #regex,
                            name: #name,
                            version: #version,
                            category: #category,
                            url: #url,
                            producer: #producer,
                        }
                    }
                });

                writeln!(
                    &mut out_data,
                    "{}",
                    quote::quote! {
                        pub static #upper: [Element; #count] = [
                            #( #data_iter , )*
                        ];
                    }
                )?;
            }
            Ok(Data::Map(map)) => {
                let count = map.len();

                let data_iter = map.into_iter().map(|(name, brand)| {
                    let Brand {
                        regex,
                        device,
                        model,
                        models,
                    } = brand;

                    let device = if let Some(device) = device {
                        quote::quote! {
                            Some(#device)
                        }
                    } else {
                        quote::quote! { None }
                    };

                    let model = if let Some(model) = model {
                        quote::quote! {
                            Some(#model)
                        }
                    } else {
                        quote::quote! { None }
                    };

                    let models = if let Some(models) = models {
                        let models_iter = models.into_iter().map(|model| {
                            let Model { regex, model } = model;

                            quote::quote! {
                                Model {
                                    regex: #regex,
                                    model: #model,
                                }
                            }
                        });

                        quote::quote! {
                            Some(&[ #( #models_iter , )* ])
                        }
                    } else {
                        quote::quote! { None }
                    };

                    quote::quote! {
                        (#name, Brand {
                            regex: #regex,
                            device: #device,
                            model: #model,
                            models: #models,
                        })
                    }
                });

                writeln!(
                    &mut out_data,
                    "{}",
                    quote::quote! {
                        pub static #upper: [(&str, Brand); #count] = [
                            #( #data_iter , )*
                        ];
                    }
                )?;
            }
            Err(err) => {
                eprintln!(
                    "error parsing `{}` at `{}`",
                    out_file.display(),
                    err.path().to_string()
                );

                return Err(err.into_inner().into());
            }
        }
    }

    fs::write(
        &manifest_dir
            .join("..")
            .join("device-detector")
            .join("src")
            .join("data.rs"),
        &out_data,
    )?;

    Ok(())
}

fn new_migration(name: &str, folder: &str) -> std::io::Result<()> {
    fs::create_dir_all(folder)?;

    let dt = Utc::now();
    let file_prefix = dt.format("%Y%m%d%H%M%S").to_string();

    let mut path = PathBuf::new();

    path.push(folder);
    path.push(&format!("{}_{}.sql", file_prefix, name.replace(' ', "_")));

    println!("Creating {}", path.display());

    let mut file = File::create(&path)?;

    file.write_all("-- Add migration script here\n".as_bytes())?;

    Ok(())
}
