use crate::dates::CurrentQuarter;
use chrono::Utc;
use leptos::*;

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
    let timezone = move || timezone().parse::<chrono_tz::Tz>().unwrap();

    // Get timestamp with selected timezone
    let timestamp_tz = move || timestamp().with_timezone(&timezone());
    let timestamp_tz_str = move || timestamp_tz().to_rfc2822();

    // Get current quarter
    let current_quarter = move || {
        CurrentQuarter::new(timestamp_tz())
            // TODO: unwrap
            .unwrap()
    };

    // Completed percentage of quarter
    let percentage = move || current_quarter().percentage_completed();

    // Pretty display of quarter durations
    let quarter_elapsed = move || current_quarter().pretty_duration_since_start().to_string();
    let quarter_remaining = move || current_quarter().pretty_duration_left().to_string();

    view! { cx,
        <div>{timestamp_tz_str}</div>

        <label for="file">"Quarter progress:"</label>
        <progress id="file" max="100" value={percentage}>{percentage}"%"</progress>
        <div>{percentage}"%"</div>

        <div>"We are in "{current_quarter().name()}</div>
        <div>"Current quarter started " {quarter_elapsed} " ago"</div>
        <div>"Next quarter in "{quarter_remaining}</div>
    }
}

/// Select a timezone
#[component]
fn TimezoneSelector(cx: Scope) -> impl IntoView {
    // List possible timezones
    let timezones = chrono_tz::TZ_VARIANTS
        .into_iter()
        .filter(|tz| tz.name().contains('/'))
        .filter(|tz| !tz.name().starts_with("Etc"))
        .map(|timezone| timezone.name());

    view! {cx,
        <div for="choose-timezone">"Choose timezone"</div>
        <select name="choose-timezone" id="choose-timezone">
            <option value="UTC">"UTC (Default)"</option>
            {
                timezones
                .map(|tz| view! { cx, <option value={tz}>{tz}</option>})
                .collect_view(cx)
            }
        </select>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let now = Utc::now;

    let (timezone, set_timezone) = create_signal(cx, "UTC".to_string());
    let (timestamp, set_timestamp) = create_signal(cx, now());

    // Refresh time every second
    set_interval(
        move || set_timestamp(now()),
        std::time::Duration::from_secs(1),
    );

    view! { cx,
        <TimezoneSelector on:change=move |ev| { set_timezone(event_target_value(&ev)); } />
        <QuarterProgress timestamp timezone />
    }
}

fn main() {
    mount_to_body(|cx| view! { cx,  <App/>  })
}
