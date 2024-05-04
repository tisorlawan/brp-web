use self::model::UserDates;
use crate::{
    auth::User,
    brp::{
        books::{get_day_plan, Book},
        content::{ChapterDispatcher, ChapterError, IndonesianBible},
        model::UserReadings,
    },
    errors::ApiError,
    utils::today_naive_date,
    view::{
        self,
        hx::HxCfg,
        pages::login::redirect_login,
        ui::{
            button::{ui_button, ButtonCfg},
            datepicker::{ui_datepicker, DatePickerCfgBuilder},
            modal::{ui_modal, ModalCfg},
            Color,
        },
    },
    AppState,
};
use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::IntoResponse,
    Form,
};
use chrono::{Datelike, Duration, NaiveDate};
use maud::{html, Markup};
use serde::{de::Error, Deserialize, Deserializer, Serialize};

pub mod books;
pub mod content;
pub mod model;

pub async fn page_brp(
    State(state): State<AppState>,
    user: Option<User>,
) -> Result<impl IntoResponse, ApiError> {
    match user {
        Some(user) => {
            let dates = UserDates::from_user_or_set_default(&state.db, user.id).await;
            tracing::trace!("dates: {:?}", dates);

            let readings = UserReadings::from_user(&state.db, user.id).await;
            tracing::trace!("readings: {:?}", readings);
            Ok(view::pages::page(
                "Index",
                page(user, readings, dates.start_date, dates.offset).await?,
            ))
        }
        None => Ok(redirect_login()),
    }
}

async fn page(
    _profile: User,
    readings: UserReadings,
    start_date: NaiveDate,
    offset: i64,
) -> Result<Markup, ApiError> {
    let reading_date = today_naive_date() + Duration::days(offset);
    let day_diff = (reading_date - start_date).num_days() + 1;

    Ok(html! {
        div class="flex justify-center" {
            div class="flex flex-col justify-between items-center h-screen gap-2 pt-4 px-4 pb-4 min-w-60 border border-border shadow shadow-foreground/10 shadow-md" {
                div {
                    form
                        hx-post="/dates"
                        id="date-form"
                        hx-target="#readings"
                        hx-include="[name='reading_idx']"
                        hx-select-oob="#chapter-content"
                    {
                        div class="flex flex-col gap-1"{
                            label for="start-date-picker"
                                class="font-semibold text-sm"
                                { "Start date" }
                            (ui_datepicker(
                                DatePickerCfgBuilder::new()
                                    .with_id("start-date-picker")
                                    .with_name("start_date")
                                    .with_value(&fmt_naivedate(start_date))
                                    .with_script("on changeDate js htmx.trigger('#date-form', 'submit') end")
                                    .build()
                            ))
                        }
                        div class="flex flex-col gap-1 mt-4"{
                            label for="current-date-picker"
                                class="font-semibold text-sm"
                                { "Currently reading" }
                            (ui_datepicker(
                                DatePickerCfgBuilder::new()
                                    .with_id("current-date-picker")
                                    .with_name("date")
                                    .with_script("on changeDate js htmx.trigger('#date-form', 'submit') end")
                                    .with_value(&fmt_naivedate(reading_date))
                                    .build()
                            ))
                        }
                    }
                    (fragment_readings_rows(&readings, day_diff, Some(0)))
                }

                div class="flex gap-2 bg-red-100 w-full" {
                    (ui_button(html!{
                            span { "Logout" }
                        },
                        &ButtonCfg::new()
                            .with_color(Color::Default)
                            .with_cn("w-full"),
                        &HxCfg::new()
                            .with_post("/logout")
                    ))
                }
            }

            @let info = get_day_plan(readings.readings.first().unwrap(), day_diff).0;
            (fragment_chapter_content(info.book, info.chapter as usize).await?)
        }
    })
}

async fn fragment_chapter_content(book: Book, chapter: usize) -> Result<Markup, ChapterError> {
    tracing::trace!("fragment_chapter_content");
    let bib = IndonesianBible.get_chapter(&book, chapter).await?;
    tracing::trace!("successfully get the bible chapter");
    Ok(html! {
        div id="chapter-content" class="border border-border bg-background-100 w-[850px] pb-4 px-4 text-wrap max-h-screen h-screen flex flex-col" {
            div class="flex justify-center font-bold py-2 mt-2"{
                (bib.title)
            }
            div class="overflow-y-auto flex-shrink border border-border p-4 scrollbar-thin scrollbar-thumb-foreground/70 scrollbar-track-foreground/10" {
                @for (idx, ch) in bib.verses.verse.iter().enumerate() {
                    div class="mb-3" {
                        @if let Some(ref title) = ch.title {
                            span class="font-bold pt-3 pb-2 block" { (title) }
                        }
                        div class="text-wrap" {
                            sup class="mr-1 text-foreground/50 font-extrabold"{ (idx + 1) }
                            span {
                                (ch.text)
                            }
                        }
                    }
                }
            }
        }
    })
}

#[derive(Debug, Deserialize)]
pub struct DatesRequest {
    #[serde(deserialize_with = "deserialize_naive_date")]
    start_date: NaiveDate,

    #[serde(deserialize_with = "deserialize_naive_date")]
    date: NaiveDate,

    reading_idx: usize,
}

fn deserialize_naive_date<'de, D>(d: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;
    NaiveDate::parse_from_str(&s, "%d %B %Y").map_err(|_| D::Error::custom("wrong date format"))
}

pub async fn post_dates(
    State(state): State<AppState>,
    user: Option<User>,
    Form(form): Form<DatesRequest>,
) -> Result<Markup, ApiError> {
    match user {
        Some(user) => {
            let offset = (form.date - today_naive_date()).num_days();
            let diff = (form.date - form.start_date).num_days() + 1;

            let readings = UserReadings::from_user(&state.db, user.id).await;
            let info = get_day_plan(&readings.readings[form.reading_idx], diff).0;

            UserDates::set(&state.db, user.id, form.start_date, offset).await;
            Ok(html! {
                (fragment_readings_rows(&readings, diff, Some(form.reading_idx)))
                (fragment_chapter_content(info.book, info.chapter as usize).await?)
            })
        }
        None => Ok(redirect_login()),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChapterQuery {
    book: String,
    chapter: usize,
    index: usize,
}

pub async fn get_q_chapter(
    user: Option<User>,
    State(state): State<AppState>,
    Query(q): Query<ChapterQuery>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::trace!("get_q_chapter {:?}", q);
    match user {
        Some(user) => {
            let dates = UserDates::from_user_or_set_default(&state.db, user.id).await;
            let reading_date = today_naive_date() + Duration::days(dates.offset);
            let day_diff = (reading_date - dates.start_date).num_days() + 1;

            let readings = UserReadings::from_user(&state.db, user.id).await;

            if q.book.starts_with("Mark") {
                return Ok(error_modal("Error", "wow").into_response());
            }

            Ok(html! {
                (fragment_readings_rows(&readings, day_diff, Some(q.index)))
                (
                    match  fragment_chapter_content(q.book.parse::<Book>().unwrap(), q.chapter).await {
                        Ok(e) => e,
                        Err(_) => {
                            return Ok(error_modal("Internal Server Error", "Error fetching chapter content").into_response());
                        }

                    }
                )
            }
            .into_response())
        }
        None => Ok(redirect_login().into_response()),
    }
}

pub fn redirect_index() -> Markup {
    html! {
        head {
            meta http-equiv="Refresh" content="0; URL=/";
        }
    }
}

fn error_modal(title: &str, content: &str) -> (HeaderMap, Markup) {
    tracing::trace!("error_modal");
    ui_modal(
        &ModalCfg::new()
            .auto_open()
            .with_content(html! {
                div class="bg-white px-4 pb-4 pt-5 sm:p-6 sm:pb-4" {
                    div class="sm:flex sm:items-start"{
                        div
                            class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-red-100 sm:mx-0 sm:h-10 sm:w-10" {
                            svg class="h-6 w-6 text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                                stroke="currentColor" aria-hidden="true" {
                                path stroke-linecap="round" stroke-linejoin="round"
                                    d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" {}
                            }
                        }
                        div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left" {
                            h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title" {(title)}
                            div class="mt-2"{
                                p class="text-sm text-gray-500"{ (content) }
                            }
                        }
                    }
                }
            })
            .with_footer(html! {
                div class="bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6" {
                    button type="button"
                        class="inline-flex w-full justify-center rounded-md bg-red-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-red-500 sm:ml-3 sm:w-auto" {
                        "Deactivate"
                    }
                    button type="button"
                        x-on:click="open = false"
                        class="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:mt-0 sm:w-auto" {
                        "Cancel"
                    }
                }
            })
    )
}

fn fragment_readings_rows(
    readings: &UserReadings,
    day_diff: i64,
    active_idx: Option<usize>,
) -> Markup {
    tracing::trace!("fragment_reading_rows");

    html! {
        div id="readings" class="w-full flex flex-col gap-2" {
            h2 class="font-bold text-md mb-3 mt-4" { "Readings" }

            input type="hidden" name="reading_idx" value=(active_idx.unwrap_or(0));
            @for (idx, (info, _)) in readings.readings.iter().map(|s| get_day_plan(s, day_diff)).enumerate() {
                @let is_active = matches!(active_idx, Some(i) if i == idx);

                @let color = if is_active { Color::Default } else {Color::Alternative};

                @let vals = serde_json::to_string(&ChapterQuery{book: info.book.clone().to_string(), chapter: info.chapter as usize, index: idx}).expect("serializable struct");
                @let hx_builder = {
                    let mut hx_builder = HxCfg::new();
                    if !is_active {
                         hx_builder = hx_builder.with_get("/q")
                            .with_vals(&vals)
                            .with_target("#chapter-content")
                            .with_swap("outerHTML")
                            .with_script("on htmx:responseError(detail) log detail.xhr.response")
                            .with_select_oob("#readings");
                    }
                    hx_builder
                };

                (ui_button(
                    html! {
                        div class="flex gap-4 w-full justify-between text-sm font-normal" {
                            span { (&info.book) }
                            span { (info.chapter) }
                        }
                    },
                    &ButtonCfg::new()
                        .with_color(color)
                        .with_id(&format!("{}-{}", &info.book, info.chapter).replace(' ', "_")),
                    &hx_builder
                ))
            }
        }
    }
}

fn fmt_naivedate(date: NaiveDate) -> String {
    format!("{}/{}/{}", date.day0() + 1, date.month0() + 1, date.year())
}
