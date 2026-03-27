//! Camera capture for the recamera SDK.
//!
//! This crate wraps the CVI MPI video pipeline (Sensor → VI → ISP → VPSS →
//! VENC) and exposes a safe Rust API for configuring the camera, starting and
//! stopping video streams, and capturing individual frames.
//!
//! On non-riscv64 hosts, all hardware operations return an error so that the
//! rest of the SDK can still compile and be tested.

use recamera_core::{Error, FrameData, ImageFormat, Resolution, Result};

/// Video channel selector.
///
/// Each channel corresponds to a VPSS output group on the CVI pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Channel {
    /// CH0 — raw RGB888 output from the ISP/VPSS.
    Raw,
    /// CH1 — JPEG-compressed output from VENC.
    Jpeg,
    /// CH2 — H.264-encoded video stream from VENC.
    H264,
}

/// Configuration for the camera pipeline.
///
/// Use the [`Default`] implementation for a sensible starting point
/// (1920×1080, 30 fps, JPEG channel).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CameraConfig {
    /// Capture resolution (width × height in pixels).
    pub resolution: Resolution,
    /// Target frame rate in frames per second.
    pub fps: u32,
    /// Which video channel to capture from.
    pub channel: Channel,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            resolution: Resolution::new(1920, 1080),
            fps: 30,
            channel: Channel::Jpeg,
        }
    }
}

/// A single captured video frame.
///
/// Wraps a [`FrameData`] value from `recamera-core` and provides convenient
/// accessor methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    /// The underlying frame data.
    pub data: FrameData,
}

impl Frame {
    /// Frame width in pixels.
    pub fn width(&self) -> u32 {
        self.data.width
    }

    /// Frame height in pixels.
    pub fn height(&self) -> u32 {
        self.data.height
    }

    /// The pixel/encoding format of this frame.
    pub fn format(&self) -> ImageFormat {
        self.data.format
    }

    /// The raw bytes of the frame (pixel data or encoded bitstream).
    pub fn as_bytes(&self) -> &[u8] {
        &self.data.data
    }

    /// Capture timestamp in milliseconds since an unspecified epoch.
    pub fn timestamp_ms(&self) -> u64 {
        self.data.timestamp_ms
    }
}

/// Camera handle for capturing frames from the CVI video pipeline.
///
/// Create one with [`Camera::new`], then call [`Camera::start_stream`] before
/// capturing frames with [`Camera::capture`].
#[derive(Debug)]
pub struct Camera {
    /// Current camera configuration.
    config: CameraConfig,
    /// Whether the camera is currently streaming.
    streaming: bool,
}

impl Camera {
    /// Create a new camera handle with the given configuration.
    ///
    /// On non-riscv64 platforms this always returns an error because the
    /// camera hardware is not available.
    #[cfg(not(target_arch = "riscv64"))]
    pub fn new(_config: CameraConfig) -> Result<Self> {
        Err(Error::Camera(
            "camera hardware not available on this platform".into(),
        ))
    }

    /// Create a new camera handle with the given configuration.
    ///
    /// Initialises the CVI MPI sensor, VI, ISP, and VPSS subsystems.
    #[cfg(target_arch = "riscv64")]
    pub fn new(config: CameraConfig) -> Result<Self> {
        // TODO: Initialise sensor via CVI_MIPI_SetSensorXXX
        // TODO: CVI_SYS_Init / CVI_VB_Init
        // TODO: CVI_VI_SetDevAttr / CVI_VI_EnableDev / CVI_VI_CreatePipe / CVI_VI_StartPipe
        // TODO: CVI_ISP_Init / CVI_ISP_Run (in a thread)
        // TODO: CVI_VPSS_CreateGrp / CVI_VPSS_SetChnAttr / CVI_VPSS_EnableChn / CVI_VPSS_StartGrp
        Ok(Self {
            config,
            streaming: false,
        })
    }

    /// Start the video stream.
    ///
    /// After this call, [`Camera::capture`] can be used to retrieve frames.
    #[cfg(not(target_arch = "riscv64"))]
    pub fn start_stream(&mut self) -> Result<()> {
        Err(Error::Camera(
            "camera hardware not available on this platform".into(),
        ))
    }

    /// Start the video stream.
    ///
    /// After this call, [`Camera::capture`] can be used to retrieve frames.
    #[cfg(target_arch = "riscv64")]
    pub fn start_stream(&mut self) -> Result<()> {
        // TODO: CVI_VENC_CreateChn / CVI_VENC_StartRecvFrame (for Jpeg/H264 channels)
        self.streaming = true;
        Ok(())
    }

    /// Stop the video stream.
    ///
    /// After this call, [`Camera::capture`] will return an error until
    /// [`Camera::start_stream`] is called again.
    #[cfg(not(target_arch = "riscv64"))]
    pub fn stop_stream(&mut self) -> Result<()> {
        Err(Error::Camera(
            "camera hardware not available on this platform".into(),
        ))
    }

    /// Stop the video stream.
    ///
    /// After this call, [`Camera::capture`] will return an error until
    /// [`Camera::start_stream`] is called again.
    #[cfg(target_arch = "riscv64")]
    pub fn stop_stream(&mut self) -> Result<()> {
        // TODO: CVI_VENC_StopRecvFrame / CVI_VENC_DestroyChn
        self.streaming = false;
        Ok(())
    }

    /// Capture a single frame from the active stream.
    ///
    /// Returns an error if the camera is not currently streaming.
    #[cfg(not(target_arch = "riscv64"))]
    pub fn capture(&self) -> Result<Frame> {
        Err(Error::Camera(
            "camera hardware not available on this platform".into(),
        ))
    }

    /// Capture a single frame from the active stream.
    ///
    /// Returns an error if the camera is not currently streaming.
    #[cfg(target_arch = "riscv64")]
    pub fn capture(&self) -> Result<Frame> {
        if !self.streaming {
            return Err(Error::Camera("camera is not streaming".into()));
        }
        // TODO: CVI_VPSS_GetChnFrame / CVI_VENC_SendFrame / CVI_VENC_GetStream
        // TODO: Build FrameData from the raw buffer returned by MPI
        todo!("capture not yet implemented — requires CVI MPI sysroot")
    }

    /// Returns a reference to the current camera configuration.
    pub fn config(&self) -> &CameraConfig {
        &self.config
    }

    /// Returns `true` if the camera is currently streaming.
    pub fn is_streaming(&self) -> bool {
        self.streaming
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_camera_config() {
        let cfg = CameraConfig::default();
        assert_eq!(cfg.resolution, Resolution::new(1920, 1080));
        assert_eq!(cfg.fps, 30);
        assert_eq!(cfg.channel, Channel::Jpeg);
    }

    #[test]
    fn frame_accessors() {
        let frame = Frame {
            data: FrameData {
                data: vec![0xAA, 0xBB, 0xCC],
                width: 640,
                height: 480,
                format: ImageFormat::Jpeg,
                timestamp_ms: 42,
            },
        };
        assert_eq!(frame.width(), 640);
        assert_eq!(frame.height(), 480);
        assert_eq!(frame.format(), ImageFormat::Jpeg);
        assert_eq!(frame.as_bytes(), &[0xAA, 0xBB, 0xCC]);
        assert_eq!(frame.timestamp_ms(), 42);
    }

    #[test]
    #[cfg(not(target_arch = "riscv64"))]
    fn camera_new_fails_on_non_riscv64() {
        let result = Camera::new(CameraConfig::default());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err
            .to_string()
            .contains("camera hardware not available on this platform"));
    }
}
