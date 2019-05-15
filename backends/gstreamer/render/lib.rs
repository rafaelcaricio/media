//! `Render` is a trait to be used by GStreamer's backend player
//!
//! The purpose of this trait is to provide different accelerated
//! video renders.
//!
//! By default, the player will use a rendering mechanism based on
//! mapping the raw video into CPU memory, but it might be other
//! rendering mechanism. The main target for this trait are
//! OpenGL-based render mechanisms.
//!
//! Each platform (Unix, MacOS, Windows) might offer an implementation
//! of this trait, so the player could setup a proper GStreamer
//! pipeline, and handle the produced buffers.
//!
extern crate gstreamer as gst;
extern crate gstreamer_video as gst_video;

extern crate servo_media_player as sm_player;

pub trait Render {
    /// Returns `True` if the render implementation uses any version
    /// or flavor of OpenGL
    fn is_gl(&self) -> bool;

    /// Returns the Player's `Frame` to be consumed by the API user.
    ///
    /// The implementator of this method will map the `buffer`,
    /// according the `info`, to the rendering appropiate
    /// structure. In the case of OpenGL-based renders, the `Frame`,
    /// instead of the raw data, will transfer the texture ID.
    ///
    /// # Arguments
    ///
    /// * `buffer` -  the GStreamer buffer to map
    /// * `info` - buffer's video information
    fn build_frame(
        &self,
        buffer: gst::Buffer,
        info: gst_video::VideoInfo,
    ) -> Result<sm_player::frame::Frame, ()>;

    /// Sets the proper *video-sink* to GStreamer's `pipeline`, this
    /// video sink is simply a decorator of the passed `appsink`.
    ///
    /// # Arguments
    ///
    /// * `appsink` - the appsink GStreamer element to decorate
    /// * `pipeline` - the GStreamer pipeline to set the video sink
    fn build_video_sink(
        &self,
        appsink: &gst::Element,
        pipeline: &gst::Element,
    ) -> Result<(), sm_player::PlayerError>;
}