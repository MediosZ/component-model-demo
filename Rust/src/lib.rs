mod bindings;
use bindings::demo::md_renderer::render::render;
use bindings::demo::word_filter::filter::filter;

pub use bindings::wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

struct Component;

bindings::export!(Component with_types_in bindings);

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(_request: IncomingRequest, outparam: ResponseOutparam) {
        let hdrs = Fields::new();
        let resp = OutgoingResponse::new(hdrs);
        let body = resp.body().expect("outgoing response");
        ResponseOutparam::set(outparam, Ok(resp));

        let out = body.write().expect("outgoing stream");
        let s = "Hello, wasi:http world!\n";
        let s = filter(s);
        let s = render(&s);
        out.blocking_write_and_flush(s.as_bytes())
            .expect("writing response");

        drop(out);
        OutgoingBody::finish(body, None).unwrap();
    }
}
