//! Actor implement in small size of code with convenient API.
//!
//! # Example
//!
//! ```
//! use client::ws::session::{Context, SessionTrait};
//! use serde::{Deserialize, Serialize};
//! use wasm_bindgen::JsValue;
//!
//! pub struct A;
//!
//! #[derive(Serialize, Deserialize)]
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
//!     fn receive(&mut self, msg: String, _: &Context<Self>) -> (&str, JsValue) {
//!         println!("received from server: {}", msg);
//!         ("message", JsValue::from(msg))
//!     }
//! }
//! ```
//!
//! # For js callback
//!
//! - `start`: when websocket is connected for the first time.
//! - `reconnect`: when websocket is reconnected due to disconnection for some reason.
//! - `disconnect`: when websocket is disconnected for any reason except stopping connection.
//! - `stop`: when websocket connection is totally stopped.

use crate::prelude::*;
use js_sys::JsString;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};

/// Trait to be implemented to use `Session`
///
/// This trait helps you to use websocket in wasm. This trait is based on actor model,
/// so it can handle on message at a time.
pub trait SessionTrait: Sized + Send + 'static {
    type Sender: Serialize + Send + 'static;

    /// It will connect websocket to `/ws/{tag}`
    fn tag() -> &'static str;

    /// This function will be called when the actor is started.
    fn started(&mut self, _: &Context<Self>) {}

    /// This function will be called when the connection is reopened.
    fn reconnected(&mut self, _: &Context<Self>) {}

    /// This function should not be overridden.
    /// This function starts the Session.
    fn start(self) -> Result<Session<Self>> {
        Session::start(self)
    }

    /// This function starts the Session with default value.
    fn start_default() -> Result<Session<Self>>
    where
        Self: Default,
    {
        Self::default().start()
    }

    /// This function will be called when websocket is disconnected.
    fn disconnected(&mut self, _: &Context<Self>) {}

    /// This function will be called when the actor is stopped.
    fn stopped(&mut self, _: &Context<Self>) {}

    /// This function handles the message from js.
    fn handle(&mut self, msg: Self::Sender, ctx: &Context<Self>) {
        ctx.send(serde_json::to_string(&msg).unwrap());
    }

    /// This function will be called when message is received from server.
    /// Callback function will be called as a result of this function.
    fn receive(&mut self, msg: String, _: &Context<Self>) -> (&str, JsValue);
}

struct Wrap<T> {
    inner: T,
}

unsafe impl<T> Send for Wrap<T> {}

enum Message<T>
where
    T: SessionTrait,
{
    ReStarted,
    Send(String),
    Handle(T::Sender),
    Receive(String),
    SetCallback(String, Wrap<Function>),
    Reconnect,
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
    pub fn restarted(&self) {
        let _ = self.sender.send(Message::ReStarted);
    }

    pub fn send(&self, msg: String) {
        let _ = self.sender.send(Message::Send(msg));
    }

    pub fn handle(&self, msg: T::Sender) {
        let _ = self.sender.send(Message::Handle(msg));
    }

    pub fn receive(&self, msg: String) {
        let _ = self.sender.send(Message::Receive(msg));
    }

    pub fn set_callback<S: AsRef<str>>(&self, tag: S, func: Function) {
        let _ = self
            .sender
            .send(Message::SetCallback(tag.as_ref().to_owned(), Wrap { inner: func }));
    }

    pub fn reconnect(&self) {
        let _ = self.sender.send(Message::Reconnect);
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

        let ctx = context.clone();
        thread::spawn(move || {
            let mut callback: HashMap<String, Function> = HashMap::new();
            let mut unsent_msg: HashMap<String, Vec<JsValue>> = HashMap::new();

            let call = |tag: &str,
                        val: JsValue,
                        callback: &mut HashMap<String, Function>,
                        unsent_msg: &mut HashMap<String, Vec<JsValue>>| {
                let tag = tag.to_owned();

                if let Some(func) = callback.get(&tag) {
                    let _ = func.call1(&JsValue::null(), &val);
                } else if let Some(msg_list) = unsent_msg.get_mut(&tag) {
                    msg_list.push(val);
                } else {
                    unsent_msg.insert(tag, vec![val]);
                }
            };

            inner.started(&ctx);
            call("start", JsValue::null(), &mut callback, &mut unsent_msg);

            'outer: loop {
                let ws = WebSocket::new(&*url).unwrap();

                let ctx_c = ctx.clone();
                let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Ok(msg) = e.data().dyn_into::<JsString>() {
                        ctx_c.receive(String::from(msg));
                    }
                }) as Box<dyn FnMut(MessageEvent)>);
                ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                onmessage.forget();

                let ctx_c = ctx.clone();
                let onerror = Closure::wrap(Box::new(move |_| {
                    ctx_c.reconnect();
                }) as Box<dyn FnMut(MessageEvent)>);
                ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));
                onerror.forget();

                let ctx_c = ctx.clone();
                let onclose = Closure::wrap(Box::new(move |_| {
                    ctx_c.stop();
                }) as Box<dyn FnMut(MessageEvent)>);
                ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
                onclose.forget();

                let ctx_c = ctx.clone();
                let onopen = Closure::wrap(Box::new(move |_| {
                    ctx_c.restarted();
                }) as Box<dyn FnMut(MessageEvent)>);
                ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
                onopen.forget();

                'inner: loop {
                    if let Ok(msg) = receiver.recv() {
                        match msg {
                            Message::ReStarted => {
                                inner.reconnected(&ctx);
                                call("reconnect", JsValue::null(), &mut callback, &mut unsent_msg);
                            }
                            Message::Send(msg) => {
                                let _ = ws.send_with_str(&*msg);
                            }
                            Message::Handle(msg) => {
                                inner.handle(msg, &ctx);
                            }
                            Message::Receive(msg) => {
                                let (tag, val) = inner.receive(msg, &ctx);
                                call(tag, val, &mut callback, &mut unsent_msg);
                            }
                            Message::SetCallback(tag, func) => {
                                let func = func.inner;
                                if let Some(msg_list) = unsent_msg.get(&tag) {
                                    for msg in msg_list.iter() {
                                        let _ = func.call1(&JsValue::null(), &msg);
                                    }
                                }
                                unsent_msg.remove(&tag);
                                callback.insert(tag, func);
                            }
                            Message::Reconnect => {
                                inner.disconnected(&ctx);
                                let _ = ws.close();
                                call("disconnect", JsValue::null(), &mut callback, &mut unsent_msg);
                                break 'inner;
                            }
                            Message::Stop => {
                                inner.stopped(&ctx);
                                let _ = ws.close();
                                call("stop", JsValue::null(), &mut callback, &mut unsent_msg);
                                break 'outer;
                            }
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

    pub fn on<S: AsRef<str>>(&self, tag: S, func: Function) {
        self.context.set_callback(tag, func);
    }
}
