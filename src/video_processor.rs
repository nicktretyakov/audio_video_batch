use ffmpeg_next::{
    format::{self, Pixel},
    frame, media,
    software::scaling::{self, Flags},
    Error,
};
use std::path::Path;

pub fn extract_frames(video_path: &Path, output_dir: &Path) -> Result<Vec<f64>, Error> {
    ffmpeg_next::init()?;

    let mut ictx = format::input(&video_path)?;
    let video_stream = ictx
        .streams()
        .best(media::Type::Video)
        .ok_or(Error::StreamNotFound)?;

    let video_stream_index = video_stream.index();
    let context_decoder =
        ffmpeg_next::codec::context::Context::from_parameters(video_stream.parameters())?;
    let mut decoder = context_decoder.decoder().video()?;

    let mut scaler = scaling::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )?;

    let mut timestamps = Vec::new();
    let mut frame_index = 0;

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;
            let mut decoded = frame::Video::empty();

            while decoder.receive_frame(&mut decoded).is_ok() {
                let timestamp = packet.pts().unwrap_or(0) as f64
                    * stream.time_base().numerator() as f64
                    / stream.time_base().denominator() as f64;
                timestamps.push(timestamp);

                let mut rgb_frame = frame::Video::empty();
                scaler.run(&decoded, &mut rgb_frame)?;

                let frame_path = output_dir.join(format!("frame_{:04}.png", frame_index));
                image::save_buffer(
                    &frame_path,
                    rgb_frame.data(0),
                    rgb_frame.width(),
                    rgb_frame.height(),
                    image::ColorType::Rgb8,
                )
                .map_err(|e| Error::Other { error: Box::new(e) })?;

                frame_index += 1;
            }
        }
    }

    decoder.send_eof()?;
    Ok(timestamps)
}
