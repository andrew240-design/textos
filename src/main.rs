use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};

const RUTA: &str = "Archivo.txt";

/// 1. Crea el archivo (si ya existe lo sobrescribe) y escribe contenido.
fn crear_y_escribir(ruta: &str, contenido: &str) -> io::Result<()> {
    let mut archivo = File::create(ruta)?;
    archivo.write_all(contenido.as_bytes())?;
    Ok(())
}

/// 2. Lee todo el archivo y lo regresa como String.
fn leer(ruta: &str) -> io::Result<String> {
    fs::read_to_string(ruta)
}

/// 3. Anexa texto al final del archivo sin borrar lo anterior.
fn anexar(ruta: &str, texto: &str) -> io::Result<()> {
    let mut archivo = OpenOptions::new()
        .append(true) // escribe al final
        .open(ruta)?; // el archivo debe existir
    writeln!(archivo, "{}", texto)?;
    Ok(())
}

/// 4. Elimina el archivo.
fn eliminar(ruta: &str) -> io::Result<()> {
    fs::remove_file(ruta)
}

fn main() -> io::Result<()> {
    // Crear y escribir
    crear_y_escribir(RUTA, "Hola desde Rust!\nEste es un texto de PRUEBA.\n")?;
    println!("[OK] Archivo '{}' creado y escrito.", RUTA);

    // Leer
    println!("\n--- Contenido inicial ---");
    println!("{}", leer(RUTA)?);

    // Anexar
    anexar(RUTA, "Esta linea fue anexada al final.")?;
    println!("[OK] Texto anexado.");

    // Leer de nuevo para ver el cambio
    println!("\n--- Contenido despues de anexar ---");
    println!("{}", leer(RUTA)?);

    // Eliminar
    eliminar(RUTA)?;
    println!("[OK] Archivo '{}' eliminado.", RUTA);

    // Comprobar que ya no existe
    match leer(RUTA) {
        Ok(_) => println!("El archivo aun existe."),
        Err(e) => println!("Confirmado, ya no existe: {}", e),
    }

    Ok(())
}