use crate::models::Worklog;
use futures::future;
use rand::prelude::*;

pub fn get_color() -> String {
    let mut rng = thread_rng();
    let r = rng.gen_range(0..150);
    let g = rng.gen_range(0..150);
    let b = rng.gen_range(0..150);
    format!("rgba({}, {}, {}, 0.4)", r, g, b)
}

// pub async fn del_worklog(
//     org_id: &String,
//     token: &String,
//     issue: &String,
//     worklog: &String)
//     -> Result<i32, reqwest::Error> {
//     let client = reqwest::Client::new();
//     let url = format!("https://api.tracker.yandex.net/v2/issues/{}/worklog/{}", issue, worklog);
//     Ok(client.delete(url)
//         .header("X-Org-ID", org_id)
//         .header("Authorization", format!("OAuth {}", token))
//         .send()
//         .await?
//         .status()
//     )
// }

pub async fn get_weekends(
    date_from: &String,
    date_to: &String)
    -> Result<Vec<i8>, reqwest::Error> {
    let client = reqwest::Client::new();
    client.get("https://isdayoff.ru/api/getdata")
        .query(&[
            ("date1", date_from),
            ("date2", date_to),
        ])
        .send()
        .await?
        .text()
        .await
        .and_then(|r| Ok(r.chars()
            .map(|c| c.to_digit(10).unwrap_or(0) as i8)
            .collect()))
}


pub async fn get_total_count(
    org_id: &String,
    yandex_id: &str,
    token: &String,
    date_from: &String,
    date_to: &String)
    -> Result<i32, reqwest::Error> {
    let client = reqwest::Client::new();
    Ok(client.get("https://api.tracker.yandex.net/v2/worklog")
        .header("X-Org-ID", org_id)
        .header("Authorization", format!("OAuth {}", token))
        .query(&[
            ("createdBy", yandex_id),
            ("perPage", "1"),
            ("start", &format!("from:{}", date_from)),
            ("start", &format!("to:{}", date_to)),
        ])
        .send()
        .await?
        .headers()
        .get("X-Total-Count")
        .and_then(|r| Some(r.to_str().unwrap().parse::<i32>().unwrap_or(0)))
        .unwrap_or(0)
    )
}

pub async fn get_worklog(
    org_id: &String,
    yandex_id: &str,
    token: &String,
    date_from: &String,
    date_to: &String,
    pages: &i32,
    total: &i32)
    -> Result<Vec<Worklog>, reqwest::Error> {
    let pages = (1..*pages + 1).collect::<Vec<i32>>();

    let client = &reqwest::Client::new();

    let bodies: Vec<Result<Vec<Worklog>, reqwest::Error>> = future::join_all(
        pages.into_iter()
            .map(|page| {
                async move {
                    let resp: Vec<Worklog> = client
                        .get("https://api.tracker.yandex.net/v2/worklog")
                        .header("X-Org-ID", org_id)
                        .header("Authorization", format!("OAuth {}", token))
                        .query(&[
                            ("page", page.to_string()),
                            ("perPage", total.to_string()),
                            ("createdBy", yandex_id.to_string()),
                            ("start", format!("from:{}", date_from)),
                            ("start", format!("to:{}", date_to)),
                        ])
                        .send()
                        .await?
                        .json()
                        .await?;
                    Ok(resp)
                }
            })
    )
        .await;

    let mut worklog_list: Vec<Worklog> = Vec::new();
    for b in bodies {
        match b {
            Ok(b) => worklog_list.extend(b),
            Err(e) => eprintln!("Got an error: {}", e),
        }
    }
    Ok(worklog_list)
}