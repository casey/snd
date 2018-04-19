extern crate cpal; 
extern crate rand;

#[cfg(test)]
extern crate float_cmp;

mod synthesizer;

use synthesizer::{Synthesizer, Voice, VoiceKind};

use cpal::{EventLoop, StreamData, UnknownTypeOutputBuffer};

// series of events, each event is just the beginning of a bar
// generates new events, 
// add attributes (which audio sample to play)
// bass?

fn main() {
  let mut synth = Synthesizer::new();

  synth.voice(Voice::new().amplitude(0.5).frequency(220.0).kind(VoiceKind::Saw));

  let event_loop = EventLoop::new();

  let device = cpal::default_output_device()
      .expect("no output device available");

  let format = device.default_output_format()
      .unwrap();

  let stream_id = event_loop.build_output_stream(&device, &format)
      .unwrap();

  event_loop.play_stream(stream_id);

  let mut samples_written: u64 = 0;

  event_loop.run(move |_stream_id, stream_data| {
    let mut buffer = if let StreamData::Output{buffer} = stream_data {
      if let UnknownTypeOutputBuffer::F32(buffer) = buffer {
        buffer
      } else {
        panic!("got non f32 buffer");
      }
    } else {
      panic!("got StreamData::Input, which we didn't request.");
    };

    for elem in buffer.iter_mut() {
      let time = (samples_written / format.channels as u64) as f64 / format.sample_rate.0 as f64;
      *elem = synth.sample(time) as f32;
      samples_written += 1;
    }
  });
}
