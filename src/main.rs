use std::env;
use url::Url;

fn main() {

    // Syntax : ./sget [--noExec] [--outFile path] url
    // Example : ./sget --noExec --outFile /home/jyotsna/scripts https://cdn.jsdelivr.net/npm/vue/dist/vue.js 

    let args: Vec<String> = env::args().collect();
    let url = Url::parse(&args[(args.len()) - 1]).expect("Please provide a valid URL with the correct scheme (http/https/www etc.)");
    println!("URL is: {}", url);

    let mut lowercase_element;
    let mut out_file = false;
    for element in args.iter() {
        lowercase_element = element.to_lowercase();
        if  lowercase_element == "--noexec"{
            println!("Don't execute the script");
        }
        if out_file {
            println!("output file path is {}",element);
            out_file = false;
        }
        if  lowercase_element == "--outfile"{
            out_file = true;
        }
    }
}

