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
        print_template_name_list().await;
    } else if matches.is_present("templates") {
        let template_name_list = matches
            .values_of("templates")
            .map(|template| template.collect::<Vec<_>>()).unwrap();
        print_gitignore(template_name_list).await;
    } else {
        println!("{}", matches.usage());
    }
}

async fn print_template_name_list() {
    let gtn_result = gitignore_io::get_template_names().await;
    match gtn_result {
        Ok(templates) => {
            for template in templates {
                println!("{}", template);
            }
        }
        Err(error) => {
            println!("Problem getting list of templates: {}", error.to_string())
        },
    };
}

async fn print_gitignore(template_name_list: Vec<&str>) {
    let gt_result = gitignore_io::get_template(&template_name_list).await;
    match gt_result {
        Ok(result) => {
            println!("{}", result);
        }
        Err(error) => {
            println!(r#"Problem getting .gitignore for "{}": {}"#, template_name_list.join(" "), error.to_string())
        },
    };
}
