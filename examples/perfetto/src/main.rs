// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use autocxx::prelude::*;

include_cpp! {
    #include "perfetto.h"
    safety!(unsafe)
    generate!("perfetto::FlushFlags")

    // include other files...

    // generate!("perfetto::base::SocketHandle")
    // generate!("perfetto::base::TaskRunner")
    // generate!("perfetto::internal::TracingMuxerImpl")
    // generate!("perfetto::CreateSocketCallback")
    // generate!("perfetto::CreateSocketAsync")
    // generate!("perfetto::Producer")
    // generate!("perfetto::ProducerEndpoint")
    // generate!("perfetto::TracingBackend")
    // generate!("perfetto::TracingProducerBackend")
    // generate!("perfetto::TracingConsumerBackend")
    // generate!("perfetto::Tracing")
    // generate!("perfetto::TracingInitArgs")
}

fn main() {
    let args = ffi::perfetto::FlushFlags::new(0);
    println!("hello");
}
