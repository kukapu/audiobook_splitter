// src/mp3_encoder.rs

use lame::Lame;

pub fn guardar_como_mp3(muestras: &[f32], frecuencia_muestreo: usize, nombre_archivo: &str) -> Result<(), std::io::Error> {
    let mut lame = Lame::new().expect("No se pudo crear el codificador LAME");
    lame.set_quality(2).expect("No se pudo establecer la calidad");
    lame.set_sample_rate(frecuencia_muestreo as u32);

    lame.set_channels(2); // Ajusta según sea mono o estéreo  
    lame.init_params().expect("No se pudieron inicializar los parámetros de LAME");

    let pcm_left = samples.clone(); // Clona el buffer para el canal izquierdo
    let pcm_right = samples; // Usa el buffer original para el canal derecho

    let mut mp3_buffer = vec![0; muestras.len() * 5 / 4 + 7200]; // Tamaño recomendado por LAME
    let tamaño_mp3 = lame.encode(&pcm_left, &pcm_right, &mut mp3_buffer).expect("Error al codificar MP3");
    mp3_buffer.truncate(tamaño_mp3);
    let mp3_buffer_bytes = unsafe {
        std::slice::from_raw_parts(mp3_buffer.as_ptr() as *const u8, mp3_buffer.len() * std::mem::size_of::<i16>())
    };
    std::fs::write(nombre_archivo, &mp3_buffer_bytes)?;


    Ok(())
}
