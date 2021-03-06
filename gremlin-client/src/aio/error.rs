use crate::GremlinError;
use async_tungstenite::tungstenite;
use futures::channel::mpsc::SendError;

impl From<tungstenite::error::Error> for GremlinError {
    fn from(e: tungstenite::error::Error) -> GremlinError {
        GremlinError::WebSocketAsync(e)
    }
}

impl From<&tungstenite::error::Error> for GremlinError {
    fn from(e: &tungstenite::error::Error) -> GremlinError {
        let error = match e {
            tungstenite::error::Error::AlreadyClosed => tungstenite::error::Error::AlreadyClosed,
            tungstenite::error::Error::ConnectionClosed => {
                tungstenite::error::Error::ConnectionClosed
            }
            _ => return GremlinError::Generic(format!("Error from ws {}", e)),
        };
        GremlinError::WebSocketAsync(error)
    }
}

impl From<SendError> for GremlinError {
    fn from(e: SendError) -> GremlinError {
        GremlinError::ChannelSend(e)
    }
}
