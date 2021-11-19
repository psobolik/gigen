mod gitignore_io;

use clap::{Arg, App};

#[tokio::main]
async fn main() {
    let matches = App::new("gigen")
        .version("0.1")
        // .author("Paul Sobolik")
        .about("Generate .gitingore file using gitignore.io API")
        .arg(Arg::with_name("list")
            .short("l")
            .long("list")
            .help("List available templates"))
        .arg(Arg::with_name("templates")
            .multiple(true)
            .last(true))
        .get_matches();

    if matches.is_present("list") {
        get_template_name_list().await;
    } else if matches.is_present("templates") {
        let template_name_list = matches
            .values_of("templates")
            .map(|template| template.collect::<Vec<_>>()).unwrap();
        get_template(template_name_list).await;
    } else {
        println!("{}", matches.usage());
    }
}

async fn get_template_name_list() {
    let gtn_result = gitignore_io::get_template_names().await;
    match gtn_result {
        Ok(templates) => {
            for template in templates {
                println!("{}", template);
            }
        }
        Err(error) => {
            let msg = error.to_string();
            println!("Problem getting template names: {}", msg)
        },
    };
}

async fn get_template(template_name_list: Vec<&str>) {
    let gt_result = gitignore_io::get_template(template_name_list).await;
    match gt_result {
        Ok(result) => {
            println!("{}", result);
        }
        Err(error) => {
            let msg = error.to_string();
            println!("Problem getting template: {}", msg)
        },
    };
}
/*
#[tokio::main]
async fn main() {
    let template_names = gitignore_io::get_template_names().await;
    match template_names {
        Ok(result) => {
            for template in result {
                println!("{}", template);
            }
        }
        Err(error) => panic!("Problem getting template names {:?}", error),
    };

    let template_vec = vec!["yarn", "vagrant"];
    let template = gitignore_io::get_template(template_vec).await.unwrap();
    println!("{}", template);
}
*/
/*
mod gitignore_io;

// use std::env;
// use crate::gitignore_io::error::Error;
use clap::{Arg, App};

#[tokio::main]
async fn main() {
    let matches = App::new("gigen")
        .version("0.1")
        // .author("Paul Sobolik")
        .about("Generate .gitingore file using gitignore.io API")
        .arg(Arg::with_name("list")
            .short("l")
            .long("list")
            .help("List available templates"))
        .arg(Arg::with_name("templates")
            .multiple(true)
            .last(true))
        .get_matches();

    if matches.is_present("list") {
        let gtn_result = gitignore_io::get_template_names().await;
        match gtn_result {
            Ok(templates) => {
                for template in templates {
                    println!("{}", template);
                }
            }
            Err(error) => {
                let msg = error.to_string();
                println!("Problem getting template names: {}", msg)
            },
        };
    } else {
        let template_list = matches
            .values_of("templates")
            .map(|template| template.collect::<Vec<_>>()).unwrap();
        // let template_list = vec!["yarn", "vagrant"];
        let gt_result = gitignore_io::get_template(template_list).await;
        match gt_result {
            Ok(result) => {
                println!("{}", result);
            }
            Err(error) => {
                let msg = error.to_string();
                println!("Problem getting template: {}", msg)
            },
        };
    }
}

// async fn get_template_name_list() -> Result<Vec<String>, Error> {
//
// }
//
// async fn get_template(template_name_list: Vec<String>) -> Result(String, Error) {
//
// }
*/
