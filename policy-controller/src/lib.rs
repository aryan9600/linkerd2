#![deny(warnings, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod admin;
pub mod admission;

pub use linkerd_policy_controller_grpc as grpc;
pub use linkerd_policy_controller_k8s_api as api;
pub use linkerd_policy_controller_k8s_index as k8s;
