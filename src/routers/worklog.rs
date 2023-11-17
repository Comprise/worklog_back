use actix_web::{get, web, HttpResponse, Responder, Result, error, delete};
use crate::middleware::AuthorizationService;
use crate::config::Config;
use crate::db::DbPool;
use crate::models::{DataDurations, Datasets, User, WorklogQuery, YandexToken, DeleteWorklog};
use crate::worklog::{get_total_count, get_worklog, get_color, get_weekends, del_worklog};
use std::collections::{BTreeMap, HashMap};
use chrono::{Duration, NaiveDate};
use reqwest::StatusCode;

#[get("/worklog")]
async fn worklog(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    auth: AuthorizationService,
    query: web::Query<WorklogQuery>,
) -> Result<impl Responder> {
    let query = query.into_inner();
    let pool = pool.into_inner();
    let config = config.into_inner();

    let date_from = query.date_from;
    let date_to = query.date_to;
    let duration = date_to - date_from;

    let duration_days: i64 = duration.num_days();
    if duration_days < 0 { Err(error::ErrorBadRequest("incorrect date"))? }
    if duration_days > 180 { Err(error::ErrorBadRequest("period too much"))? }

    let pool_clone = pool.clone();
    let user = web::block(move || {
        let mut conn = pool_clone.get()?;
        User::find_by_id(&mut conn, &auth.user_id)
    })
        .await?
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
        .ok_or_else(|| error::ErrorUnauthorized("User not found"))?;

    let pool_clone = pool.clone();
    let user_clone = user.clone();
    let token = web::block(move || {
        let mut conn = pool_clone.get()?;
        YandexToken::get_by_user_id(&mut conn, &user_clone)
    })
        .await?
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let date_to_from_yandex = date_to + Duration::days(1);

    let (total, pages, total_count) = if duration_days > 7 {
        let total_count = get_total_count(
            &config.org_id, &user.yandex_id, &token.access_token,
            &date_from.to_string(), &date_to_from_yandex.to_string())
            .await
            .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

        let total = 50;
        let pages =
            if total_count > total {
                total_count / total + if (total_count % total) == 0 { 0 } else { 1 }
            } else { 1 };
        (total, pages, total_count)
    } else {
        let total = 1000;
        let pages = 1;
        (total, pages, 1)
    };

    let worklog = if total_count > 0 {
        get_worklog(&config.org_id, &user.yandex_id, &token.access_token,
                    &date_from.to_string(), &date_to_from_yandex.to_string(),
                    &pages, &total)
            .await
            .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
    } else { Vec::new() };


    let mut data_durations: BTreeMap<NaiveDate, Option<Vec<DataDurations>>> = BTreeMap::new();
    let mut date_from_clone = date_from.clone();

    while date_from_clone != date_to_from_yandex {
        data_durations.insert(date_from_clone, None);
        date_from_clone += Duration::days(1);
    }

    let mut colors: HashMap<String, String> = HashMap::new();
    let mut total_duration = 0;
    let mut count = 0;
    for wl in &worklog {
        if data_durations.contains_key(&wl.start) {
            let mut value = data_durations[&wl.start]
                .clone()
                .unwrap_or(Vec::new());
            let wl_copy = wl.clone();

            let color = if colors.contains_key(&wl_copy.issue.id) {
                colors[&wl_copy.issue.id].clone()
            } else {
                let color = get_color();
                colors.insert(wl_copy.issue.id.clone(), color.clone());
                color
            };
            total_duration += wl_copy.duration;

            let data_duration = DataDurations {
                duration: wl_copy.duration,
                worklog: wl_copy.id,
                key: wl_copy.issue.key,
                issue: wl_copy.issue.id,
                title: wl_copy.issue.display,
                background_colors: color,
            };
            value.push(data_duration);
            let value_len = value.len();
            data_durations.insert(wl.start, Option::from(value));

            count = if count < value_len { value_len } else { count }
        }
    }

    let mut datasets: Vec<Vec<Option<DataDurations>>> = Vec::new();

    for i in 0..count {
        let mut dataset: Vec<Option<DataDurations>> = Vec::new();
        for (_, data) in &data_durations {
            match data {
                None => { dataset.push(None) }
                Some(data) => {
                    if i < data.len() {
                        dataset.push(Option::from(data[i].clone()));
                    } else {
                        dataset.push(None);
                    }
                }
            }
        }
        datasets.push(dataset);
    }

    let weekends = get_weekends(
        &date_from.format("%Y%m%d").to_string(),
        &date_to.format("%Y%m%d").to_string())
        .await
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let labels: Vec<NaiveDate> = data_durations.keys().cloned().collect();

    if weekends.len() != labels.len() {
        Err(error::ErrorInternalServerError("Error"))?
    }
    let datasets_data = Datasets {
        datasets,
        labels,
        weekends,
        total_duration,
    };

    Ok(HttpResponse::Ok().json(datasets_data))
}

#[delete("/worklog")]
async fn delete_worklog(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    auth: AuthorizationService,
    data: web::Json<DeleteWorklog>,
) -> Result<impl Responder> {
    let pool = pool.into_inner();
    let config = config.into_inner();
    let data = data.into_inner();

    let pool_clone = pool.clone();
    let user = web::block(move || {
        let mut conn = pool_clone.get()?;
        User::find_by_id(&mut conn, &auth.user_id)
    })
        .await?
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
        .ok_or_else(|| error::ErrorUnauthorized("User not found"))?;

    let pool_clone = pool.clone();
    let user_clone = user.clone();
    let token = web::block(move || {
        let mut conn = pool_clone.get()?;
        YandexToken::get_by_user_id(&mut conn, &user_clone)
    })
        .await?
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let status_code = del_worklog(
        &config.org_id,
        &token.access_token,
        &data.issue,
        &data.worklog)
        .await
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    println!("{}", status_code.as_str());

    if status_code != StatusCode::NO_CONTENT {
        Err(error::ErrorInternalServerError("Error"))?
    };

    Ok(HttpResponse::NoContent())
}