use super::*;
use chrono_tz::{Europe, Tz};
use std::collections::HashMap;

const TZ: Tz = Europe::Helsinki;

struct Context {
    pub year: i32,
}
const CTX: Context = Context { year: 2019 };

lazy_static! {
    static ref EXAMPLE_EVENTS_BY_LINE: HashMap<&'static str, Event> = hashmap! {
        "    25.04               diplomityö Janille viimeistä kommentointia varten"
        => {
            let local = TZ.ymd(CTX.year, 4, 25).with_timezone(&Local);
            let date =
                DateVariant::Date(local);
            Event {
                date, description: "diplomityö Janille viimeistä kommentointia varten".to_owned()
            }
        },
        "ma  25.04    23:59      [](PRML kirjan laina-aika päättyy)"
        => {
            let local = TZ.ymd(CTX.year, 4, 25).and_hms(23, 59, 0).with_timezone(&Local);
            let date = DateVariant::DateTime(local);
            Event {
                date, description: "[](PRML kirjan laina-aika päättyy)".to_owned()
            }
        },
        "	25.-28.7			Saskia's Music Festival in late July"
        => {
            let local_start = TZ.ymd(CTX.year, 7, 25).and_hms(6, 0, 0).with_timezone(&Local);
            let local_end = TZ.ymd(CTX.year, 7, 28).and_hms(23, 59, 0).with_timezone(&Local);
            let date = DateVariant::TimeSpan(local_start, local_end);
            Event {
                date, description: "Saskia's Music Festival in late July".to_owned()
            }
        },
    };
}

#[test]
fn example_events_parse_correct() {
    env_logger::init();

    for (key, value) in EXAMPLE_EVENTS_BY_LINE.iter() {
        assert_eq!(Event::from_str(key, CTX.year).unwrap(), *value,);
    }
}