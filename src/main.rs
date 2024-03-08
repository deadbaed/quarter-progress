use crate::dates::CurrentQuarter;
use chrono::{Datelike, Utc};
use leptos::*;
use leptos_router::*;

mod dates;

/// Display progress of a quarter
#[component]
fn QuarterProgress(
    /// UTC timestamp
    #[prop(into)]
    timestamp: Signal<chrono::DateTime<Utc>>,
) -> impl IntoView {
    // Get query "tz" into a String
    let query = use_query_map();

    // Parse query to timezone, UTC if it does not exist or does not parse
    let timezone = move || {
        query
            .with(|query| query.get("tz").cloned())
            .unwrap_or("UTC".into())
            .parse::<chrono_tz::Tz>()
            .unwrap_or(chrono_tz::UTC)
    };

    log::debug!("Using timezone {}", timezone());

    // Get timestamp with selected timezone
    let timestamp_tz = move || timestamp.get().with_timezone(&timezone());
    let date = move || timestamp_tz().format("%A %d %B").to_string();
    let time = move || timestamp_tz().format("%T %Z").to_string();

    // Get current quarter
    let current_quarter = move || {
        CurrentQuarter::new(timestamp_tz())
            // TODO: unwrap
            .unwrap()
    };
    let current_quarter_name = move || current_quarter().name();

    // Completed percentage of quarter
    let percentage = move || format!("{:.6}%", current_quarter().percentage_completed());
    let progress_bar_width =
        move || format!("width: {:.1}%", current_quarter().percentage_completed());

    // Pretty display of quarter durations
    let quarter_elapsed = move || current_quarter().pretty_duration_since_start().to_string();
    let quarter_remaining = move || current_quarter().pretty_duration_left().to_string();

    view! {
        <div class="space-y-2">
            <div class="text-4xl">{date}</div>
            <div class="text-3xl tracking-wide">{time}</div>
        </div>

        <div class="space-y-2">
            <div class="text-2xl">{current_quarter_name}" "{move || timestamp_tz().year()}</div>

            <div>
                <div class="w-full bg-gray-200">
                    <div class="bg-blue-500 h-8" style={progress_bar_width}></div>
                </div>
            </div>

            <div class="text-lg">{percentage}" completed"</div>
        </div>

        <div class="space-y-4 text-lg text-justify">
            <div>{current_quarter_name}" started " {quarter_elapsed} " ago"</div>
            <div>{current_quarter_name}" ends in "{quarter_remaining}</div>
        </div>

    }
}

#[component]
fn TimezoneSelector() -> impl IntoView {
    // Put UTC at the top
    let timezones_utc = ["UTC"];

    // List other possible timezones
    let timezones = chrono_tz::TZ_VARIANTS
        .into_iter()
        .filter(|tz| tz.name().contains('/'))
        .filter(|tz| !tz.name().starts_with("Etc"))
        .map(|timezone| timezone.name());

    let all_options =
        // Combine all options together
        timezones_utc
            .into_iter()
            .chain(timezones)
            // Create their view
            .map(|tz| {
                // Link with timezone as query
                let encoded_timezone = url::form_urlencoded::byte_serialize(tz.as_bytes()).collect::<String>();
                let uri = github_pages_route(format!("?tz={}", encoded_timezone));
                view! { <p class="my-1"><Link uri=uri label=tz /></p> }
            })
            .collect_view();

    view! {
        <div class="text-3xl">"Select your timezone"</div>
        <div>{all_options}</div>
    }
}

fn github_pages_route<S: Into<String>>(url: S) -> String {
    if cfg!(debug_assertions) {
        url.into()
    } else {
        format!("/quarter-progress{}", url.into())
    }
}

#[component]
fn Link(uri: String, label: &'static str) -> impl IntoView {
    view! {
        <A class="rounded-lg px-3 py-2 font-medium hover:bg-slate-100 hover:text-slate-900" href=uri>{label}</A>
    }
}

#[component]
fn App() -> impl IntoView {
    let now = Utc::now;
    let (timestamp, set_timestamp) = create_signal(now());

    // Refresh time every second
    set_interval(
        move || set_timestamp.set(now()),
        std::time::Duration::from_secs(1),
    );

    view! {
        <Router>
            <main>
                <Routes>
                    <Route path=github_pages_route("/*") view=move || view! {
                        <nav class="flex justify-center items-center space-x-4 mb-4">
                            <Link uri=github_pages_route("/timezone") label="Change timezone" />
                        </nav>
                        <div class="space-y-12">
                            <QuarterProgress timestamp=timestamp />
                        </div>
                    }/>
                    <Route path=github_pages_route("/timezone") view=TimezoneSelector />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    log::debug!("Base URL is: {}", github_pages_route("/"));

    mount_to_body(|| {
        view! {
            <div class="flex-grow">
                <main class="container mx-auto px-4 py-8 max-w-3xl">
                    <div class="text-3xl mb-4">"Quarter Progress"</div>
                    <App />
                </main>
            </div>

            <footer class="container mx-auto px-4 py-8">
                <div>
                    "Quarter Progress was made in Q2 2023 with ❤️ by "
                    <a class="underline" href="https://philippeloctaux.com">"phil"</a>
                </div>
                <div>
                    "Project made to learn "
                    <a class="underline" href="https://leptos.dev">"leptos"</a>
                    ", see the "
                    <a class="underline" href="https://github.com/x4m3/quarter-progress">"source code"</a>
                </div>
            </footer>
        }
    })
}
