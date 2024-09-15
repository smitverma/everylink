mod brute_functions;
use crate::brute_functions::check_if_page_exists;

use std::collections::HashSet;
use clap::{Arg, Command, command};
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::io::{stdout, Write};

fn main() {
    // let links_array : Vec<String>;

    let matches = command!().subcommand_required(false)
        .subcommand(Command::new("crawl")
            .about("Crawl the website to discover webpages upto a certain depth (default:2)")
        )
        .subcommand(Command::new("brute")
            .about("Brute-force the URL to discover webpages or directories against a wordlist (default:evlink.lst) upto a certain depth (default:2)")
        )
        .arg(Arg::new("url")
            .short('u')
            .long("url").global(true)
            // .required(true)
            .help("The URL to be used as the starting point"))
        .arg(Arg::new("wordlist")
            .short('w')
            .long("wlist")
            .help("The wordlist to be used for directory brute-forcing"))
        .arg(Arg::new("depth")
            .short('d').global(true)
            .long("depth")
            .help("The depth till which links are traversed"))
        .get_matches();

    match matches.subcommand_name() {
        Some("crawl") => {
            if let Some(url) = matches.get_one::<String>("url") {
                println!("URL: {}", url);

                if let Some(depth) = matches.get_one::<String>("depth") {
                    println!("Depth: {}", depth);

                    // let url= "https://blog.logrocket.com/making-http-requests-rust-reqwest/";

                    let mut links_visited= HashSet::new();
                    let mut links_array= HashSet::new();
                    let depth_int = depth.parse::<i32>().unwrap();

                    match get_page(url, 0, depth_int, &mut links_visited, &mut links_array) {
                        Ok(_) => {}
                        Err(_) => {}
                    };

                    println!("\nCrawling complete!\nTotal links found : {}\nYour links are :", links_array.len());
                    for link_in_array in links_array.clone() {
                        println!("{}", link_in_array)
                    }
                }
                else {
                    println!("Note : Depth not specified.\nDefault is 2. Specify using -d or --depth.");
                    // let url= "https://blog.logrocket.com/making-http-requests-rust-reqwest/";

                    let depth=2;
                    let mut links_visited= HashSet::new();
                    let mut links_array= HashSet::new();

                    match get_page(url, 0, depth, &mut links_visited, &mut links_array) {
                        Ok(_) => {}
                        Err(_) => {}
                    };

                    println!("\nCrawling complete!\nTotal links found : {}\nYour links are :", links_array.len());
                    for link_in_array in links_array.clone() {
                        println!("{}", link_in_array)
                    }
                    return;
                }

            }
            else {
                println!("Error : URL not specified.\nSpecify using -u or --url.");
                return;
            }

        }
        Some("brute") => {
            if let Some(url) = matches.get_one::<String>("url") {
                println!("URL: {}", url);

                let mut url_prepped = trim_url(url);
                if url_prepped.ends_with('/') != true {
                    url_prepped = url_prepped + "/";
                }

                let wlist_words= ["", "login", "register", "signup", "adfmri2omin"];
                /// Some pages like Instagram do not give an error code 404 even on entering garbage sub directories
                /// Check on burp suite - if redirection, show it as such
                    /// How to detect redirection programmatically? Status code? Learning the page not found page and detecting it?
                /// Also implement feature to check URL with large gibberish subdirectories, if all give back 200 / OK, then probably such a case
                    /// So give disclaimer in such a case
                /// Check - How do gobuster and dirb react to such sites?

                // if let Some(wlist) = matches.get_one::<String>("wlist") {
                //     println!("Wordlist: {}", url); // TO DO
                // }
                // else {
                //     wlist_words = ["login", "register", "signup", "admin"];
                // }

                for wlist_word in wlist_words {
                    let _ = check_if_page_exists(url_prepped.clone()+wlist_word);
                }

            }
            else {
                println!("Error : URL not specified.\nSpecify using -u or --url.");
                return;
            }
        }
        _ => {
            println!("Error : Incorrect or no subcommand specified!")
        }
    }
}


fn get_page(url: &str, depth_count: i32, max_depth: i32, links_visited: &mut HashSet<String>, links_array: &mut HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {

    // println!("\n\n######################\nVisiting link {}: {}",count, url);
    // print!("Links visited : ");
    // for link_visited in links_visited.clone() {
    //     print!("{}, ", link_visited)
    // }
    // print!("\n({}) Links in array : ", links_array.len());
    // for link_in_array in links_array.clone() {
    //     print!("{}, ", link_in_array)
    // }
    // // println!("\n{}", links_visited.len());
    // println!("\n######################\n\n");


    if links_visited.contains(&trim_url(url).to_string()) != true {
        links_visited.insert(trim_url(url).to_string());
        // println!("Depth count : {}, Max depth : {}", depth_count, max_depth);
        if depth_count < max_depth {
            let client = Client::new();
            let response = client.get(url).send()?;
            let body = response.text()?;

            let document = Html::parse_document(&body);
            let selector = Selector::parse("a").unwrap();

            for element in document.select(&selector) {
                if let Some(href) = element.value().attr("href") {
                    if href.starts_with('#') != true {
                        let mut link: String = href.to_string();
                        if href.starts_with('/') {
                            link = extract_base_url(url) + href;
                            // println!("Found link: {}", link);
                        }
                        // else {
                        //     println!("Found link: {}", link);
                        // }
                        if extract_main_domain(&link).eq(&extract_main_domain(url)) {
                            link = trim_url(&link);
                            if links_array.contains(&link) != true {
                                links_array.insert(link);
                            }
                            // println!("Internal link : {}", link);

                            // print!("\nLinks in array : ");
                            // for link_in_array in links_array.clone() {
                            //     print!("{}, ", link_in_array)
                            // }
                            // println!();
                        }
                        // else {
                        //     println!("{}\n{}", extract_main_domain(&link).unwrap(), &extract_main_domain(url).unwrap());
                        // }
                    }
                }
                print!("\n({}) Links in array : ", links_array.len());
                for link_in_array in links_array.clone() {
                    print!("{}, ", link_in_array)
                }
            }
        }
    }

    // println!("\n\n######################\n");
    // print!("Links visited : ");
    // for link_visited in links_visited.clone() {
    //     print!("{}, ", link_visited)
    // }
    // println!(" {}", links_visited.len());
    // println!("######################\n\n");

    if depth_count + 1 < max_depth {
        let links_to_process: Vec<_> = links_array.iter().cloned().collect();
        for link in links_to_process {
            update_links_count(links_array.len());
            match get_page(&link, depth_count + 1, max_depth, links_visited, links_array) {
                Ok(_) => {}
                Err(_) => {}
            };
        }
    }

    Ok(())
}

fn extract_base_url(full_url: &str) -> String {
    if let Some(scheme_pos) = full_url.find("://") {
        if let Some(domain_end) = full_url[scheme_pos + 3..].find('/') {
            let base_url = &full_url[..scheme_pos + 3 + domain_end];
            return base_url.to_string();
        } else {
            return full_url.to_string();
        }
    } else {
        return "Invalid Link".to_string();
    }
}

fn extract_main_domain(full_url: &str) -> Option<String> {
    // Remove the scheme (e.g., "http://", "https://") if present
    let url = full_url.trim_start_matches("http://")
        .trim_start_matches("https://");

    // Find the position of the first slash after the domain
    let domain_end = url.find('/').unwrap_or(url.len());

    // Extract the domain part
    let domain = &url[..domain_end];

    // Split the domain by '.'
    let parts: Vec<&str> = domain.split('.').collect();

    if parts.len() >= 2 {
        // Return the last two parts joined by a '.'
        Some(format!("{}.{}", parts[parts.len() - 2], parts[parts.len() - 1]))
    } else {
        None
    }
}

fn trim_url(url: &str) -> String {
    // println!("To trim : {}", url);
    // let urltemp = url.split_once("/?")
    //     .or_else(|| url.split_once("/#"))
    //     .map(|(start, _)| start)
    //     .unwrap_or(url)
    //     .trim_end_matches('/')
    //     .trim_end_matches('?')
    //     .to_string();
    // println!("Trimmed : {}", urltemp);
    url.split_once("/?")
        .or_else(|| url.split_once("/#"))
        .map(|(start, _)| start)
        .unwrap_or(url)
        .trim_end_matches('/')
        .trim_end_matches('?')
        .to_string()

}

fn update_links_count(count: usize) {
    print!("\rLinks discovered : {}", count);
    stdout().flush().unwrap();
}