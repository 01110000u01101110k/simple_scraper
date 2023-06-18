use std::fs;

const START_TAG_OPEN_SYMBOL: &str = "<";
const END_TAG_OPEN_SYMBOL: &str = "</";
const TAG_CLOSE_SYMBOL: &str = ">";

struct WebsiteInfoForScraping {
    part_of_attribute_to_search: String,
    search_tag: String,
    url: String,
    website_name: String
}

#[tokio::main]
async fn main() {
    let website_list: Vec<WebsiteInfoForScraping> = vec![
        WebsiteInfoForScraping {
            part_of_attribute_to_search: "".to_string(),
            search_tag: "p".to_string(),
            url: "https://doc.rust-lang.org/book/".to_string(),
            website_name: "rust_book".to_string()
        },
        WebsiteInfoForScraping {
            part_of_attribute_to_search: "class=\"toggle\"".to_string(),
            search_tag: "summary".to_string(),
            url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript".to_string(),
            website_name: "wdeveloper.mozilla.org".to_string()
        },
    ];

    let mut scraping_result: Vec<String> = Vec::new();

    for website in website_list {
        let mut response = reqwest::get(website.url.trim())
            .await.unwrap()
            .text()
            .await.unwrap();

        let part_of_attribute = website.part_of_attribute_to_search;

        let summary_tag_start = format!("{}{}", START_TAG_OPEN_SYMBOL, &website.search_tag);
        let summary_tag_end = format!("{}{}", END_TAG_OPEN_SYMBOL, &website.search_tag);

        let mut search_in_progress = true;

        scraping_result.push(format!("---------------start scraping from the {}---------------", website.website_name));
        scraping_result.push(format!("website url: {}", website.url));

        while search_in_progress {

            let find_attribute = response.find(&part_of_attribute);

            if find_attribute.is_some() {
                response = response[find_attribute.unwrap()..response.len()].to_string();

                let find_summary_tag_start = response.find(&summary_tag_start);
                let find_summary_tag_end = response.find(&summary_tag_end);

                if find_summary_tag_start.is_some() && find_summary_tag_end.is_some() {
                    response = response[find_summary_tag_start.unwrap()..response.len()].to_string();

                    let find_end_first_tag_symbol = response.find(TAG_CLOSE_SYMBOL);
                    let find_start_last_tag_symbol = response.find(END_TAG_OPEN_SYMBOL);

                    if find_end_first_tag_symbol.is_some() && find_start_last_tag_symbol.is_some() {
                        scraping_result.push(response[(find_end_first_tag_symbol.unwrap() + 1)..find_start_last_tag_symbol.unwrap()].to_string());
                    }
                    response = response[find_start_last_tag_symbol.unwrap()..response.len()].to_string();
                } else {
                    search_in_progress = false
                }
            } else {
                search_in_progress = false
            }
        }

        scraping_result.push(format!("---------------end of scraping from {}---------------", website.website_name));
    }

    println!("{:#?}", &scraping_result);

    let file_result = "result.txt";

    fs::write(file_result, format!("{:#?}", scraping_result)).unwrap();
}
