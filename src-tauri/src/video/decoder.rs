use crate::state::Frame;

pub struct H264Decoder;

impl H264Decoder {
    pub fn new() -> Result<Self, String> {
        // TODO: Initialize FFmpeg decoder
        // In production, use ffmpeg_next or a custom H.264 decoder
        Ok(Self)
    }

    pub fn decode(&self, _data: &[u8]) -> Result<Frame, String> {
        // TODO: Decode H.264 NAL units to RGB frames
        // Placeholder implementation
        Ok(Frame {
            timestamp: 0,
            width: 0,
            height: 0,
            data: Vec::new(),
        })
    }
}
