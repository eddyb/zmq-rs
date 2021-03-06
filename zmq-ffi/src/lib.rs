#![allow(dead_code)]
extern crate libc;

pub use ffi::{
    zmq_msg_t,
    zmq_pollitem_t,

    zmq_errno,
    zmq_strerror,
    zmq_version,

    zmq_ctx_new,
    zmq_ctx_term,
    zmq_ctx_shutdown,
    zmq_ctx_set,
    zmq_ctx_get,

    zmq_msg_init,
    zmq_msg_init_size,
    zmq_msg_init_data,
    zmq_msg_send,
    zmq_msg_recv,
    zmq_msg_close,
    zmq_msg_move,
    zmq_msg_copy,
    zmq_msg_data,
    zmq_msg_size,
    zmq_msg_more,
    zmq_msg_get,
    zmq_msg_set,
    zmq_msg_gets,

    zmq_socket,
    zmq_close,
    zmq_setsockopt,
    zmq_getsockopt,
    zmq_bind,
    zmq_connect,
    zmq_unbind,
    zmq_disconnect,
    zmq_send,
    zmq_send_const,
    zmq_recv,
    //zmq_sendmsg,          // Deprecated
    //zmq_recvmsg,          // Deprecated
    zmq_socket_monitor,

    zmq_has,

    zmq_poll,
    zmq_proxy,
    zmq_proxy_steerable,

    zmq_z85_encode,
    zmq_z85_decode,
    zmq_curve_keypair,
};

#[allow(non_camel_case_types)]
mod ffi {
    use libc::{
        size_t,
        uint8_t,
        c_void,
        c_int,
        c_char,
        c_ulong,
        c_uchar,
        c_short,
        c_long,
    };

    include!("ffi.rs");
}
