use std::{thread, time};
use std::env;
use std::path::{PathBuf, Path};
use scraper::{Html, Selector};
use serde_json::json;
use reqwest;
use reqwest::header;
use serde::{Deserialize, Serialize};
use csv::Writer;
use notify_rust::Notification;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut report: Vec<OrgItem> = Vec::new();

    let file_name =format!("{}.csv", chrono::offset::Local::now().format("%Y%m%d%H%M%S"));
    let file_path: PathBuf;

    if args.len() > 1 && Path::new(&args[1]).exists() {
        file_path = PathBuf::from(&args[1]).join(file_name);
    } else {
        file_path = PathBuf::from(file_name);
    }

    let mut wtr = Writer::from_path(file_path)?;

    let mut n : i32 = 1;
    let mut cnt :i32 = 51;
    while cnt > 50 {
        report.append(&mut parse(download(n).await?).await?);

        cnt = report.last().unwrap().count;
        println!("count: {}", cnt);
        thread::sleep(time::Duration::from_secs(1));
        n += 1;
    }

    let count = report.len();
    for elem in report {
        wtr.write_record(&[elem.symbol, elem.count.to_string()])?;
    }
    wtr.flush()?;

    Notification::new()
    .summary("Orgstats")
    .body(format!("{} orgs recorded", count).as_str())
    .icon("firefox")
    .show()?;

    Ok(())
}

async fn download(page_num : i32) -> Result<GetOrgsResp, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/114.0".parse().unwrap());
    headers.insert("Accept", "*/*".parse().unwrap());
    headers.insert("Accept-Language", "en-US,en;q=0.5".parse().unwrap());
    headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
    headers.insert("Origin", "https://robertsspaceindustries.com".parse().unwrap());
    headers.insert("DNT", "1".parse().unwrap());
    headers.insert("Connection", "keep-alive".parse().unwrap());
    headers.insert("Referer", "https://robertsspaceindustries.com/community/orgs/listing?sort=size_desc&search=&pagesize=12&page=1".parse().unwrap());
    headers.insert("Sec-Fetch-Dest", "empty".parse().unwrap());
    headers.insert("Sec-Fetch-Mode", "cors".parse().unwrap());
    headers.insert("Sec-Fetch-Site", "same-origin".parse().unwrap());
    headers.insert("Sec-GPC", "1".parse().unwrap());
    headers.insert("TE", "trailers".parse().unwrap());

    let json = json!({
        "sort": "size_desc",
        "search": "",
        "commitment": [],
        "roleplay": [],
        "size": [],
        "model": [],
        "activity": [],
        "language": [],
        "recruiting": [],
        "pagesize": 12,
        "page": page_num
    });

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.post("https://robertsspaceindustries.com/api/orgs/getOrgs")
        .headers(headers)
        .json(&json)
        .send().await?;

    println!("Status: {}", res.status());

    let org_resp: GetOrgsResp = serde_json::from_str(&res.text().await?)?;

    return Ok(org_resp);
}

async fn parse(org_resp : GetOrgsResp) -> Result<Vec<OrgItem>, Box<dyn std::error::Error>>{
    let mut org_vec: Vec<OrgItem> = Vec::new();

    let document = Html::parse_document(&org_resp.data.html);
    let selector = Selector::parse(r#"div.org-cell"#).unwrap();
    for elem in document.select(&selector) {
        let orgfragment = Html::parse_fragment(&elem.inner_html());

        let symbol_selector = Selector::parse(r#"span.symbol"#).unwrap();
        let count_selector = Selector::parse(r#"span.infoitem span.value"#).unwrap();

        let org_symbol = orgfragment.select(&symbol_selector).next().unwrap();

        let info_item_vec = orgfragment.select(&count_selector).collect::<Vec<_>>();

        let item = OrgItem { symbol: org_symbol.inner_html(), count: info_item_vec.get(5).unwrap().inner_html().parse::<i32>().unwrap() };
        org_vec.push(item);
    }
    return Ok(org_vec) ;
}

#[derive(Serialize, Deserialize, Debug)]
struct GetOrgsResp {
    success: i32,
    code: String,
    msg: String,
    data: OrgData
}

#[derive(Serialize, Deserialize, Debug)]
struct OrgData {
    totalrows: i32,
    html: String,
}

struct OrgItem {
    symbol: String,
    count: i32
}