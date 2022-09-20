use scraper::{Html, Selector};
use std::fs;
// based on https://kadekillary.work/post/webscraping-rust/

fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("link_scanner"))
        .unwrap();

    check_url_order(settings.get_string("url").unwrap());

    check_owner_title(settings.get_string("owner_url").unwrap());
}

fn check_url_order(url:String) {
    let resp = match reqwest::blocking::get(&url) {
        Ok(x) => x,
        Err(x) => {
            panic!("host check failed: {}", x);
        },
    };
    
    assert!(resp.status().is_success());

    let body = resp.text().unwrap();
    let frag = Html::parse_document(&body);
    let properties = Selector::parse(".property_title").unwrap();

    let mut title_order: Vec<&str> = Vec::new();
    for property in frag.select(&properties) {
        let property_title = property.text().collect::<Vec<_>>();
        for s in property_title.iter() {
            title_order.push(s.trim());
        }
    }

    let mut write_out = false;
    let prev_list: Vec<&str>;
    //println!("{:?}", title_order);
    // compare to previous run
    match fs::read("last-titles.dat") {
        Ok(x) => {
            prev_list = bincode::deserialize(&x[..]).unwrap();
            if prev_list
                .iter()
                .zip(&title_order)
                .filter(|&(a, b)| a != b)
                .count()
                > 0
            {
                eprintln!("Order changed!\nCurrent:");
                for i in title_order.iter() {
                    eprintln!("{}", i);
                }
                eprintln!("Prev:");
                for i in prev_list {
                    eprintln!("{}", i);
                }
                write_out = true;
            }
        }
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                write_out = true;
            }
            _ => panic!("Failed to read strings from last-titles.dat: {:?}", e),
        },
    };

    if write_out {
        // serialize the title_order variable, which is the new
        // sequence we just got
        if let Err(e) = fs::write("last-titles.dat", bincode::serialize(&title_order).unwrap()) {
            panic!("Failed to write string to last-titles.dat: {:?}", e);
        }
    }
}

fn check_owner_title(url: String) {
    let resp = reqwest::blocking::get(&url).unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().unwrap();
    let frag = Html::parse_document(&body);
    let owner = Selector::parse(".agentName a").unwrap();

    for owner_tag in frag.select(&owner) {
        let o = owner_tag.value().attr("title").unwrap();
        if o.contains("Owner") {
            eprintln!("Word 'Owner' appears in agent's title: {:?}",o)
        }        
    }

}