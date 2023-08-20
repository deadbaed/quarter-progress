use crate::dates::CurrentQuarter;
use chrono::{Datelike, Utc};
use leptos::*;
use leptos_use::storage::use_storage;
use serde::{Deserialize, Serialize};

mod dates;

/// Display progress of a quarter
#[component]
fn QuarterProgress(
    cx: Scope,
    /// UTC timestamp
    #[prop(into)]
    timestamp: Signal<chrono::DateTime<Utc>>,
    /// Selected timezone
    #[prop(into)]
    timezone: Signal<String>,
) -> impl IntoView {
    // TODO: unwrap
    // Parse timezone
    let timezone = move || timezone.get().parse::<chrono_tz::Tz>().unwrap();

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

    view! { cx,
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

/// Select a timezone
#[component]
fn TimezoneSelector(
    cx: Scope,
    /// Currently selected timezone
    #[prop(into)]
    timezone: Signal<String>,
) -> impl IntoView {
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
            // Add selected prop if value is currently selected
            view! { cx, <option value=tz prop:selected=move || timezone.get() == {tz}>{tz}</option>}
        })
        .collect_view(cx);

    view! {cx,
        <span for="choose-timezone">"Timezone:"</span>
        <select name="choose-timezone" id="choose-timezone" class="appearance-none mx-2 p-1 pr-4 border-2 border-gray-200 rounded-md text-sm dark:bg-slate-800">
            {all_options}
        </select>
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Settings {
    pub timezone: String,
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let now = Utc::now;
    let (timestamp, set_timestamp) = create_signal(cx, now());

    // Refresh time every second
    set_interval(
        move || set_timestamp.set(now()),
        std::time::Duration::from_secs(1),
    );

    // Settings
    let settings = Settings {
        timezone: "UTC".into(),
    };
    let (settings, set_settings, _) = use_storage(cx, "settings", settings.clone());
    let timezone = create_memo(cx, move |_| settings.get().timezone);

    view! { cx,
        <div class="space-y-8">
            <div>
                <TimezoneSelector timezone on:change=move |ev| { set_settings.update(|s| s.timezone = event_target_value(&ev)) } />
            </div>

            <div class="space-y-12">
                <QuarterProgress timestamp timezone />
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <div class="flex-grow">
                <main class="container mx-auto px-4 py-8 max-w-3xl">
                    <div class="text-xl mb-4">"Quarter Progress"</div>
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
