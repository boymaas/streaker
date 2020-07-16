use crate::route::AppRoute;
use std::time::Duration;
use yew::prelude::*;
use yew::services::interval::IntervalService;
use yew::services::Task;
use yew_router::prelude::*;

use chrono::{DateTime, TimeZone, Utc};

#[derive(Default)]
pub struct TimeRemaining {
    hours: i64,
    minutes: i64,
    seconds: i64,
}

pub struct Clock {
    countdown: TimeRemaining,
    props: Properties,
    interval_service: IntervalService,
    task: Box<dyn Task>,
}

pub enum Msg {
    Updating,
}

#[derive(Properties, Clone)]
pub struct Properties {
    pub target_time: DateTime<Utc>,
}

fn time_remaining(target_time: &DateTime<Utc>) -> TimeRemaining {
    let duration = target_time.signed_duration_since(Utc::now());
    TimeRemaining {
        hours: duration.num_hours(),
        minutes: duration.num_minutes() % 60,
        seconds: duration.num_seconds() % 60,
    }
}

impl Component for Clock {
    type Message = Msg;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let duration = Duration::from_secs(1);
        let callback = link.callback(|_| Msg::Updating);
        // NOTE when task is dropped, interval is stopped
        let mut interval_service = IntervalService::new();
        let task = interval_service.spawn(duration, callback);
        Self {
            countdown: time_remaining(&props.target_time),
            props: props,
            interval_service,
            task: Box::new(task),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Updating => {
                // lets calculate time remaining to target time
                self.countdown = time_remaining(&self.props.target_time);
            }
        }
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <div class="clock">
              <span class="hours">{ format!("{:0>2}", self.countdown.hours) }</span>
              <span class="sep">{":"}</span>
              <span class="minutes">{ format!("{:0>2}", self.countdown.minutes) }</span>
              <span class="sep">{":"}</span>
              <span class="seconds">{ format!("{:0>2}", self.countdown.seconds) }</span>
            </div>
        }
    }
}
