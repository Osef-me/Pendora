use md5::Context;

pub fn hash_md5(file_content: &str) -> Result<String, String> {
    // Crée un contexte MD5
    let mut context = Context::new();

    // Consomme directement le contenu en bytes
    context.consume(file_content.as_bytes());

    // Récupère le digest final
    let result = context.finalize();
    let hash_string = format!("{:x}", result);
    Ok(hash_string)
}
