use webrtc::api::APIBuilder;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::data_channel::RTCDataChannel;
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::data_channel::data_channel_init::RTCDataChannelInit;
use webrtc::ice_transport::ice_candidate::RTCIceCandidate;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescriptionSerde;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::interceptor_registry::Registry;
use webrtc::api::API;

use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;
use async_trait::async_trait;

/// Trait abstraction for mocking WebRTC behavior in tests
#[async_trait]
pub trait WebRTC: Send + Sync {
    async fn send_message(&self, data: &[u8]) -> Result<()>;
}

/// Represents a WebRTC client capable of establishing peer-to-peer connections.
pub struct WebRTCClient {
    pub peer_connection: Arc<RTCPeerConnection>,
    pub data_channel: Arc<RTCDataChannel>,
}

impl WebRTCClient {
    /// Creates a new WebRTC client with default configuration.
    pub async fn new() -> Result<Self> {
        // Create a MediaEngine object to configure the supported codec
        let mut m = MediaEngine::default();
        m.register_default_codecs()?;

        // Create the InterceptorRegistry
        let mut registry = Registry::new();
        registry = register_default_interceptors(registry, &mut m)?;

        // Create the API object with the MediaEngine and InterceptorRegistry
        let api = APIBuilder::new()
            .with_media_engine(m)
            .with_interceptor_registry(registry)
            .build();

        // Define ICE servers
        let config = RTCConfiguration {
            ice_servers: vec![RTCIceServer {
                urls: vec!["stun:stun.l.google.com:19302".to_string()],
                ..Default::default()
            }],
            ..Default::default()
        };

        // Create a new RTCPeerConnection
        let peer_connection = Arc::new(api.new_peer_connection(config).await?);

        // Create a data channel
        let data_channel = peer_connection
            .create_data_channel("data", Some(RTCDataChannelInit {
                ..Default::default()
            }))
            .await?;

        Ok(Self {
            peer_connection,
            data_channel,
        })
    }

    /// Creates an SDP offer for initiating a connection.
    pub async fn create_offer(&self) -> Result<RTCSessionDescription> {
        let offer = self.peer_connection.create_offer(None).await?;
        self.peer_connection.set_local_description(offer.clone()).await?;
        Ok(offer)
    }

    /// Sets the remote SDP answer.
    pub async fn set_remote_description(&self, sdp: RTCSessionDescription) -> Result<()> {
        self.peer_connection.set_remote_description(sdp).await?;
        Ok(())
    }

    /// Adds a remote ICE candidate.
    pub async fn add_ice_candidate(&self, candidate: RTCIceCandidate) -> Result<()> {
        self.peer_connection.add_ice_candidate(candidate).await?;
        Ok(())
    }
}

/// Implementation of the WebRTC trait for the real client.
#[async_trait]
impl WebRTC for WebRTCClient {
    async fn send_message(&self, data: &[u8]) -> Result<()> {
        self.data_channel.send(&DataChannelMessage {
            is_binary: true,
            data: data.to_vec(),
        }).await?;
        Ok(())
    }
}

 
