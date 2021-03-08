#[macro_use]
extern crate bitflags;

mod bindings;

use std::ffi::CString;
use std::time::Duration;

//AIFF OpenFile flags
bitflags! {
    pub struct OpenFlags: i32 {
        const READ_ONLY = 1;
        const WRITE_ONLY = 2;
        const AIFC = 16;
        const OPTIMIZE_SIZE = 24;
        const NOT_SEEKABLE = 32;
    }
}

pub struct AIFF {
    aiff_ref: *mut bindings::s_AIFF_Ref,
}

impl AIFF {
    /// Load AIFF file from path
    pub fn open_file(path: &str, flags: OpenFlags) -> Option<AIFF> {
        unsafe {
            let aiff_ref =
                bindings::AIFF_OpenFile(CString::new(path).unwrap().into_raw(), flags.bits());
            if !aiff_ref.is_null() {
                return Some(AIFF { aiff_ref });
            }
        }
        None
    }

    // Close libAIFF
    fn close(&self) {
        unsafe {
            bindings::AIFF_CloseFile(self.aiff_ref);
        }
    }

    /// Read `count` samples from AIFF
    pub fn read_samples(&self, count: i32) -> Option<Vec<i32>> {
        let mut samples = vec![0; count as usize];
        #[allow(unused_assignments)]
        let mut read = 0;
        unsafe {
            read = bindings::AIFF_ReadSamples32Bit(self.aiff_ref, samples.as_mut_ptr(), count);
        }
        //Error occured
        if read == -1 {
            return None;
        }
        samples.truncate(read as usize);
        Some(samples)
    }

    /// Get format details
    pub fn get_audio_format(&self) -> AIFFFormat {
        let mut format = AIFFFormat {
            samples: 0,
            channels: 0,
            sampling_rate: 0.0,
            bits_per_sample: 0,
            segment_size: 0,
        };
        unsafe {
            bindings::AIFF_GetAudioFormat(
                self.aiff_ref,
                &mut format.samples,
                &mut format.channels,
                &mut format.sampling_rate,
                &mut format.bits_per_sample,
                &mut format.segment_size,
            );
        }
        format
    }

    /// Seek to sample
    pub fn seek(&self, sample_frame: u64) {
        unsafe {
            bindings::AIFF_Seek(self.aiff_ref, sample_frame);
        }
    }

    /// Seek to duration
    pub fn seek_duration(&self, duration: Duration) {
        let format = self.get_audio_format();
        let pos = format.sampling_rate as u64 * duration.as_secs();
        self.seek(pos);
    }
}

#[derive(Debug, Clone)]
pub struct AIFFFormat {
    samples: u64,
    channels: i32,
    sampling_rate: f64,
    bits_per_sample: i32,
    segment_size: i32,
}

impl Drop for AIFF {
    fn drop(&mut self) {
        //Close file on drop
        self.close();
    }
}

#[cfg(test)]
mod tests {
    //Test if the library even works
    #[test]
    fn test_aiff() {
        use super::*;
        let aiff = AIFF::open_file("noise.aiff", OpenFlags::READ_ONLY).unwrap();
        let format = aiff.get_audio_format();
        assert_ne!(format.samples, 0);
        let samples = aiff.read_samples(format.samples as i32).unwrap();
        assert_eq!(samples.len(), format.samples as usize);
        println!("Read {} samples", samples.len());
    }
}
