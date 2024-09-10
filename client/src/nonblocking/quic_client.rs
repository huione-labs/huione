#[deprecated(
    since = "1.15.0",
    note = "Please use `huione_quic_client::nonblocking::quic_client::QuicClientConnection` instead."
)]
pub use huione_quic_client::nonblocking::quic_client::QuicClientConnection as QuicTpuConnection;
pub use huione_quic_client::nonblocking::quic_client::{
    QuicClient, QuicClientCertificate, QuicLazyInitializedEndpoint,
};
