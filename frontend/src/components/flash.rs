use std::time;
use yew::prelude::*;
use yew::services::interval::IntervalService;
use yew::services::Task;
use yew_router::prelude::*;

use chrono::{DateTime, Duration, TimeZone, Utc};

pub struct Flash {
    message: Option<String>,
    message_time: Option<DateTime<Utc>>,
    props: Properties,
    interval_service: IntervalService,
    task: Box<dyn Task>,
}

pub enum Msg {
    Updating,
}

#[derive(Properties, Clone)]
pub struct Properties {
    pub duration: Duration,
    pub message: Option<String>,
    pub callback: Callback<bool>,
}

impl Component for Flash {
    type Message = Msg;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let duration = time::Duration::from_secs(1);
        let callback = link.callback(|_| Msg::Updating);
        // NOTE when task is dropped, interval is stopped
        let mut interval_service = IntervalService::new();
        let task = interval_service.spawn(duration, callback);
        Self {
            message: None,
            message_time: None,
            props: props,
            interval_service,
            task: Box::new(task),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Updating => {
                // lets calculate time remaining to target time
                //
                if self.message_time.is_some() {
                    let duration = Utc::now().signed_duration_since(self.message_time.unwrap());
                    if duration > self.props.duration {
                        self.message = None;
                        self.message_time = None;
                        // callback to parent component to actually
                        // clear the property setting that led to the change
                        // of the message
                        self.props.callback.emit(true);
                    }
                }
            }
        }
        true
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // if the contents are different,
        // lets start displaying for a while
        self.props = props.clone();
        if props.message != self.message {
            self.message = props.message;
            self.message_time = Some(Utc::now());
            return true;
        }
        false
    }
    fn view(&self) -> Html {
        if self.message.is_some() {
            html! {
                <div class="flash">
                <span>{ self.message.as_ref().unwrap() }</span>
                </div>
            }
        } else {
            html! {}
        }
    }
}
