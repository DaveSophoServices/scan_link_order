use scraper::{Html, Selector};
// based on https://kadekillary.work/post/webscraping-rust/

fn main() {
    let mut settings = config::Config::default();
    settings
	.merge (config::File::with_name("link_scanner")).unwrap();
    
    let url = settings.get_str("url").unwrap();

    loop {
	let resp = reqwest::blocking::get(&url).unwrap();
	assert!(resp.status().is_success());

	let body = resp.text().unwrap();
	let frag = Html::parse_document(&body);
	let properties = Selector::parse(".property_title").unwrap();

	let mut title_order = Vec::new();
	for property in frag.select(&properties) {
	    let property_title = property.text().collect::<Vec<_>>();
	    for s in property_title.iter() {
		title_order.push(s.trim());
	    }
	}

	println!("{:?}", title_order);
	// compare to previous run
	
	break;
    }
    
}
