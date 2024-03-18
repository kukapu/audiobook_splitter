// src/main.rs

mod audio_processor;
mod mp3_encoder;

use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Probe;
use std::fs::File;
use std::env;
use audio_processor::detectar_silencios;
use mp3_encoder::guardar_como_mp3;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: audiobook_splitter <file>");
        std::process::exit(1);
    }
    let path = &args[1];
    let file = File::open(path)?;

    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let probed = Probe::new(mss).guess(None, Default::default())?;
    let mut format_reader = probed.format;

    // Esta es una simplificación. Deberías adaptar tu código para integrar la detección de silencios
    // y la codificación MP3 de manera que cada capítulo detectado sea procesado y guardado como un archivo MP3.
    detectar_silencios(&mut format_reader).expect("Error al detectar silencios");

    // Supongamos que detectar_silencios() ahora también devuelve información sobre los capítulos detectados,
    // podrías iterar sobre esos capítulos y usar guardar_como_mp3() aquí.
    // Por ejemplo:
    let capitulo_muestras = vec![0.0_f32; 44100]; // Ejemplo de muestras de un capítulo.
    guardar_como_mp3(&capitulo_muestras, 44100, "capitulo_1.mp3").expect("Error al guardar MP3");

    Ok(())
}
