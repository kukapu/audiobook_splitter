// src/audio_processor.rs

use symphonia::core::formats::FormatReader;
use symphonia::core::errors::Error;

// Constantes para la detección de silencios
const UMBRAL_DE_SILENCIO: f32 = 0.02; // Define el umbral de silencio (ajusta según sea necesario)
const DURACION_MINIMA_SILENCIO: usize = 44100 * 2; // Ejemplo: 2 segundos de silencio a 44100Hz

pub fn detectar_silencios(format_reader: &mut dyn FormatReader) -> Result<(), Error> {
    let mut silencio_actual = 0;
    let mut inicio_capitulo = 0;

    for packet_result in format_reader.packets() {
        let packet = packet_result?;
        let decoded = format_reader.decode(&packet)?;
        // Asegúrate de que el formato de muestra esperado es f32, de lo contrario, considera convertir o manejar adecuadamente.
        if let Some(buffer) = decoded.samples().as_slice_of::<f32>() {
            for &sample in buffer.iter() {
                if sample.abs() < UMBRAL_DE_SILENCIO {
                    silencio_actual += 1;
                } else {
                    if silencio_actual >= DURACION_MINIMA_SILENCIO {
                        println!("Capítulo detectado desde la muestra {} hasta {}", inicio_capitulo, inicio_capitulo + silencio_actual);
                        // Aquí podrías iniciar el proceso de codificación de este capítulo a MP3
                    }
                    inicio_capitulo += silencio_actual + 1;
                    silencio_actual = 0;
                }
            }
        }
    }

    Ok(())
}
