use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

// Lee una línea del teclado mostrando un mensaje antes
fn leer_linea(mensaje: &str) -> String {
    print!("{}", mensaje);
    io::stdout().flush().expect("No se pudo vaciar la salida");

    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Error al leer la entrada");
    entrada.trim().to_string()
}

// Lee varias líneas hasta que el usuario deje una línea vacía
fn leer_texto() -> String {
    println!("Escribe el texto (una línea vacía para terminar):");
    let mut texto = String::new();
    loop {
        let linea = leer_linea("> ");
        if linea.is_empty() {
            break;
        }
        texto.push_str(&linea);
        texto.push('\n');
    }
    texto
}

// 1. Crear el archivo (falla si ya existe, para no sobrescribir por error)
fn crear_archivo(ruta: &str) -> io::Result<()> {
    let contenido = leer_texto();
    let mut archivo = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(ruta)?;
    archivo.write_all(contenido.as_bytes())?;
    Ok(())
}

// 2. Verificar que existe
fn verificar_existencia(ruta: &str) {
    if Path::new(ruta).exists() {
        match fs::metadata(ruta) {
            Ok(meta) => println!("✔ El archivo '{}' existe ({} bytes).", ruta, meta.len()),
            Err(_) => println!("✔ El archivo '{}' existe.", ruta),
        }
    } else {
        println!("✘ El archivo '{}' NO existe.", ruta);
    }
}

// 3. Ver el contenido
fn mostrar_contenido(ruta: &str) {
    match fs::read_to_string(ruta) {
        Ok(contenido) => {
            println!("--- Contenido de '{}' ---", ruta);
            if contenido.is_empty() {
                println!("(vacío)");
            } else {
                print!("{}", contenido);
            }
            println!("--- Fin ---");
        }
        Err(e) => println!("No se pudo leer el archivo: {}", e),
    }
}

// 4. Modificar: agregar al final o reemplazar todo
fn modificar_archivo(ruta: &str) -> io::Result<()> {
    println!("1. Agregar texto al final");
    println!("2. Reemplazar todo el contenido");
    let opcion = leer_linea("Elige una opción: ");

    match opcion.as_str() {
        "1" => {
            let texto = leer_texto();
            let mut archivo = OpenOptions::new().append(true).open(ruta)?;
            archivo.write_all(texto.as_bytes())?;
            println!("Texto agregado.");
        }
        "2" => {
            let texto = leer_texto();
            fs::write(ruta, texto)?;
            println!("Contenido reemplazado.");
        }
        _ => println!("Opción no válida."),
    }
    Ok(())
}

// 5. Borrar y comprobar que ya no existe
// Devuelve true si el archivo fue borrado
fn borrar_archivo(ruta: &str) -> bool {
    let respuesta = leer_linea(&format!("¿Seguro que quieres borrar '{}'? (s/n): ", ruta));
    if !respuesta.eq_ignore_ascii_case("s") {
        println!("Borrado cancelado.");
        return false;
    }

    match fs::remove_file(ruta) {
        Ok(()) => {
            println!("Archivo borrado. Verificando...");
            verificar_existencia(ruta);
            !Path::new(ruta).exists()
        }
        Err(e) => {
            println!("No se pudo borrar el archivo: {}", e);
            false
        }
    }
}

fn main() {
    // Guarda el nombre del archivo con el que se está trabajando
    let mut archivo_actual: Option<String> = None;

    loop {
        println!("\n===== GESTOR DE ARCHIVO DE TEXTO =====");
        match &archivo_actual {
            Some(nombre) => println!("Archivo actual: {}", nombre),
            None => println!("Archivo actual: (ninguno)"),
        }
        println!("1. Crear archivo");
        println!("2. Verificar que existe");
        println!("3. Ver contenido");
        println!("4. Modificar archivo");
        println!("5. Borrar archivo");
        println!("0. Salir");

        let opcion = leer_linea("Elige una opción: ");

        match opcion.as_str() {
            "1" => {
                let nombre = leer_linea("Nombre del archivo (ej. notas.txt): ");
                if nombre.is_empty() {
                    println!("El nombre no puede estar vacío.");
                    continue;
                }
                match crear_archivo(&nombre) {
                    Ok(()) => {
                        println!("Archivo creado.");
                        archivo_actual = Some(nombre);
                        // Verificación automática tras crear
                        verificar_existencia(archivo_actual.as_deref().unwrap());
                    }
                    Err(e) => println!("No se pudo crear el archivo: {}", e),
                }
            }
            "2" | "3" | "4" | "5" => {
                let ruta = match &archivo_actual {
                    Some(nombre) => nombre.clone(),
                    None => {
                        println!("Primero crea un archivo (opción 1).");
                        continue;
                    }
                };

                match opcion.as_str() {
                    "2" => verificar_existencia(&ruta),
                    "3" => mostrar_contenido(&ruta),
                    "4" => {
                        if let Err(e) = modificar_archivo(&ruta) {
                            println!("Error al modificar: {}", e);
                        }
                    }
                    "5" => {
                        if borrar_archivo(&ruta) {
                            archivo_actual = None;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            "0" => {
                println!("Saliendo del programa");
                break;
            }
            _ => println!("Opción no válida."),
        }
    }
}