use reqwest::{self, Client};
use std::collections::HashMap;
use std::path::Path;
use std::{fs, io};

pub struct Framework {
    active_module: &'static str,
    entries: Vec<Entry>,
}
pub type Result<T> = ::std::result::Result<T, String>;

#[derive(Debug)]
pub enum IoError {
    Io(io::Error),
    Http(reqwest::Error),
    Puzzle(String),
}
pub type IoResult<T> = ::std::result::Result<T, IoError>;

impl From<io::Error> for IoError {
    fn from(e: io::Error) -> IoError {
        IoError::Io(e)
    }
}
impl From<reqwest::Error> for IoError {
    fn from(e: reqwest::Error) -> IoError {
        IoError::Http(e)
    }
}
impl From<String> for IoError {
    fn from(e: String) -> IoError {
        IoError::Puzzle(e)
    }
}

struct Entry {
    module: &'static str,
    url: &'static str,
    execute: Box<Fn(String) -> Result<String>>,
}

const CACHE_PATH: &'static str = "cache.ron";

#[derive(Serialize, Deserialize, Default)]
struct Cache {
    session: Option<String>,
    input_cache: HashMap<String, String>,
    output_cache: HashMap<(String, usize), String>,
}

impl Cache {
    fn load<P: AsRef<Path>>(path: P) -> Cache {
        use ron::de;
        fs::read(path)
            .ok()
            .and_then(|x| de::from_bytes(&x).ok())
            .unwrap_or_default()
    }

    fn try_store<P: AsRef<Path>>(&self, path: P) {
        use ron::ser;
        if let Ok(serialized) = ser::to_string_pretty(self, ser::PrettyConfig::default()) {
            let _ = fs::write(path, serialized);
        };
    }
}

fn id<T>(x: T) -> T {
    x
}

fn create_connection(cache: &mut Cache) -> IoResult<Client> {
    let session_key = match cache.session {
        Some(ref session_key) => session_key,
        None => {
            print!("Select a session key: ");
            use std::io::Write;
            io::stdout().flush()?;
            let mut session = String::new();
            io::stdin().read_line(&mut session)?;
            session.retain(|c| !c.is_whitespace());
            cache.session = Some(session);
            cache.session.as_ref().unwrap()
        }
    };

    use reqwest::header::{Cookie, Headers};
    let mut headers = Headers::new();
    let mut cookie = Cookie::new();
    cookie.append("session", session_key.clone());
    headers.set(cookie);

    Ok(Client::builder().default_headers(headers).build()?)
}

impl Framework {
    pub fn new() -> Self {
        Framework {
            active_module: "none",
            entries: Vec::new(),
        }
    }

    pub fn set_active_module(&mut self, s: &'static str) {
        self.active_module = s;
    }

    pub fn register<Output>(
        &mut self,
        input: &'static str,
        execute: &'static Fn(String) -> Result<Output>,
    ) where
        Output: ToString,
    {
        self.register_with_transform(input, &id, execute)
    }

    pub fn register_with_transform<Input, Output>(
        &mut self,
        input: &'static str,
        transform: &'static Fn(String) -> Input,
        execute: &'static Fn(Input) -> Result<Output>,
    ) where
        Output: ToString,
    {
        self.entries.push(Entry {
            module: self.active_module,
            url: input,
            execute: box move |input| execute(transform(input)).map(|x| x.to_string()),
        })
    }

    pub fn execute(self) -> IoResult<()> {
        let mut cache = Cache::load(CACHE_PATH);

        use std::thread::sleep;
        use std::time::Duration;

        let mut client: Option<Client> = None;
        'outer: loop {
            for entry in &self.entries {
                if cache.input_cache.contains_key(entry.url) {
                    continue;
                }

                if client.is_none() {
                    client = Some(create_connection(&mut cache)?);
                } else {
                    sleep(Duration::from_secs(1));
                }
                let client = client.as_mut().unwrap();

                println!("Retrieving: {}", entry.url);
                let mut response = client.get(entry.url).send()?;
                use reqwest::StatusCode;
                match response.status() {
                    StatusCode::Ok => {
                        cache
                            .input_cache
                            .insert(entry.url.to_string(), response.text()?);
                    },
                    StatusCode::BadRequest => {
                        println!("Bad request, session key probably expired");
                        cache.session = None;
                        continue 'outer;
                    },
                    _ => {
                        response.error_for_status()?;
                    }
                };
            }
            break;
        }

        cache.try_store(CACHE_PATH);

        let mut module_index_map = HashMap::new();
        for entry in &self.entries {
            let index = *module_index_map
                .entry(entry.module)
                .and_modify(|count| *count += 1)
                .or_insert(0usize);

            let cache_key = (entry.module.to_string(), index);
            if cache.output_cache.contains_key(&cache_key) {
                continue;
            }

            println!("Running puzzle {} - {}", entry.module, index + 1);
            let input = cache.input_cache.get(entry.url).unwrap();
            let output = (entry.execute)(input.clone())?;

            println!("Solution:\n{}\n\nIs this answer correct? (Y/N)", output);
            let mut should_save = String::new();
            io::stdin().read_line(&mut should_save)?;
            should_save.retain(|c| !c.is_whitespace());

            if should_save == "Y" || should_save == "y" {
                println!("Saved solution");
                cache.output_cache.insert(cache_key, output);
            }
        }

        cache.try_store(CACHE_PATH);

        Ok(())
    }
}
