//! Actor implement in small size of code with convenient API.
//!
//! # Example
//!
//! ```
//! use public::ws::session::{Context, SessionTrait};
//!
//! pub struct A;
//! pub struct B;
//!
//! impl SessionTrait for A {
//!     type Sender = B;
//!
//!     fn tag() -> &'static str {
//!         "example"
//!     }
//!
//!     fn started(&mut self, _: &Context<Self>) {
//!         println!("A started")
//!     }
//!
//!     fn stopped(&mut self, _: &Context<Self>) {
//!         println!("A stopped")
//!     }
//!
//!     fn receive(&mut self, msg: String, _: &Context<Self>) {
//!         println!("received from server: {}", msg);
//!     }
//! }
//! ```

use crate::prelude::*;
use js_sys::JsString;
use serde::Serialize;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};

pub trait SessionTrait: Sized + Send + 'static {
    type Sender: Serialize + Send + 'static;

    fn tag() -> &'static str;

    fn started(&mut self, _: &Context<Self>) {}

    fn start(self) -> Result<Session<Self>> {
        Session::start(self)
    }

    fn start_default() -> Result<Session<Self>>
    where
        Self: Default,
    {
        Self::default().start()
    }

    fn stopped(&mut self, _: &Context<Self>) {}

    fn handle(&mut self, msg: Self::Sender, ctx: &Context<Self>) {
        ctx.send(serde_json::to_string(&msg).unwrap());
    }

    fn receive(&mut self, msg: String, _: &Context<Self>);
}

enum Message<T>
where
    T: SessionTrait,
{
    Send(String),
    Handle(T::Sender),
    Receive(String),
    Stop,
}

pub struct Context<T>
where
    T: SessionTrait,
{
    sender: Sender<Message<T>>,
}

impl<T> Clone for Context<T>
where
    T: SessionTrait,
{
    fn clone(&self) -> Self {
        Context {
            sender: self.sender.clone(),
        }
    }
}

impl<T> Context<T>
where
    T: SessionTrait,
{
    pub fn send(&self, msg: String) {
        let _ = self.sender.send(Message::Send(msg));
    }

    pub fn handle(&self, msg: T::Sender) {
        let _ = self.sender.send(Message::Handle(msg));
    }

    pub fn receive(&self, msg: String) {
        let _ = self.sender.send(Message::Receive(msg));
    }

    pub fn stop(&self) {
        let _ = self.sender.send(Message::Stop);
    }
}

pub struct Session<T>
where
    T: SessionTrait,
{
    context: Context<T>,
}

impl<T> Clone for Session<T>
where
    T: SessionTrait,
{
    fn clone(&self) -> Self {
        Session {
            context: self.context.clone(),
        }
    }
}

impl<T> Session<T>
where
    T: SessionTrait,
{
    pub fn start(mut inner: T) -> Result<Session<T>> {
        let protocol = if window()?.location().protocol()? == "https:" {
            "wss"
        } else {
            "ws"
        };
        let host = window()?.location().host()?;
        let url = format!("{}://{}/ws/{}", protocol, host, T::tag());

        let (sender, receiver) = channel();
        let context = Context { sender };
        inner.started(&context);

        let ctx = context.clone();
        thread::spawn(move || {
            let ws = WebSocket::new(&*url).unwrap();
            let ctx_c = ctx.clone();
            let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
                if let Ok(msg) = e.data().dyn_into::<JsString>() {
                    ctx_c.receive(String::from(msg));
                }
            }) as Box<dyn FnMut(MessageEvent)>);
            ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            onmessage.forget();

            loop {
                if let Ok(msg) = receiver.recv() {
                    match msg {
                        Message::Send(msg) => {
                            let _ = ws.send_with_str(&*msg);
                        }
                        Message::Handle(msg) => {
                            inner.handle(msg, &ctx);
                        }
                        Message::Receive(msg) => {
                            inner.receive(msg, &ctx);
                        }
                        Message::Stop => {
                            inner.stopped(&ctx);
                            let _ = ws.close();
                            return;
                        }
                    }
                }
            }
        });

        Ok(Session { context })
    }

    pub fn send(&self, msg: T::Sender) {
        self.context.handle(msg);
    }
}
