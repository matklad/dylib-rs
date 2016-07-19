extern crate libloading;

use std::io::prelude::*;

use libloading::{Symbol, Library};

fn main() {
    loop {
        let plugins = PluginConfig::new();
        let app = App::new(&plugins);
        match app.repl() {
            ReplResult::Exit => return,
            ReplResult::Reload => {}
        }
    }
}


type Callback = extern fn();

struct PluginConfig {
    hi_lib: Option<Library>,
    bye_lib: Option<Library>
}

impl PluginConfig {
    fn new() -> PluginConfig {
        fn load_library_if_exists(libname: &str) -> Option<Library> {
            if std::path::Path::new(libname).exists() {
                Some(Library::new(libname).unwrap())
            } else {
                None
            }
        }

        PluginConfig {
            hi_lib: load_library_if_exists("./libs/libhi.so"),
            bye_lib: load_library_if_exists("./libs/libbye.so"),
        }
    }
}

struct App<'a> {
    hi_extension: Option<Symbol<'a, Callback>>,
    bye_extension: Option<Symbol<'a, Callback>>,
}

enum ReplResult {
    Reload,
    Exit
}

impl<'a> App<'a> {
    fn new(plugins: &PluginConfig) -> App {
        App {
            hi_extension: plugins.hi_lib.as_ref().map(|l| unsafe { l.get(b"hi\0").unwrap() }),
            bye_extension: plugins.bye_lib.as_ref().map(|l| unsafe { l.get(b"bye\0").unwrap() })
        }
    }


    fn repl(&self) -> ReplResult {
        let mut buf = String::new();
        loop {
            buf.clear();
            print!("> ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut buf).unwrap();
            match buf.trim() {
                "exit" => {
                    if let Some(ref bye) = self.bye_extension {
                        bye();
                    } else {
                        println!("bye!");
                    }
                    return ReplResult::Exit
                },
                "greet" => {
                    if let Some(ref hi) = self.hi_extension {
                        hi();
                    } else {
                        println!("Hi!");
                    }
                },
                "reload" => return ReplResult::Reload,
                _ => println!("?"),
            }
        }
    }
}



