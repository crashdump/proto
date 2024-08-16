use extism_pdk::{memory::internal::load, MemoryHandle};
use warpgate_api::SendRequestOutput;

pub fn populate_send_request_output(output: &mut SendRequestOutput) {
    if output.body.is_empty() {
        let handle = unsafe { MemoryHandle::new(output.body_offset, output.body_length) };
        let mut body = vec![0; handle.length as usize];

        load(handle, &mut body);

        output.body = body;
    }
}