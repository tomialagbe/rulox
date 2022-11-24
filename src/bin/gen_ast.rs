use std::env::args;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::{fs::File, path::PathBuf};

fn main() -> std::io::Result<()> {
    let mut args = args();
    let Some(output_dir) = args.nth(1) else {
        panic!("Missing argument <out_dir>");
    };

    // define_ast(
    //     &output_dir,
    //     "Expr",
    //     vec![
    //         "Binary   : left: T, right: T, token: Token",
    //         "Grouping : expression: T",
    //         "Literal  : value: TokenLiteral",
    //         "Unary    : operator: Token, right: T",
    //     ],
    // )?;

    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) -> std::io::Result<()> {
    let mut mod_declarations: Vec<String> = Vec::new();
    for typ in types.iter() {
        let typ_str: &str = typ;

        let Some(parts) = typ_str.split_once(":") else {
            continue;
        };

        let struct_name: &str = parts.0.trim();
        let struct_filename = struct_name.to_lowercase();
        mod_declarations.push(struct_filename.to_owned());

        // get fields
        let field_str: &str = parts.1.trim();
        let fields: Vec<&str> = field_str.split(",").map(|f| f.trim()).collect();
        println!(
            "Struct name {}, filename: {}\nFields: {:?}",
            struct_name, struct_filename, fields
        );
        let has_t: bool = fields.iter().find(|&&f| f.ends_with(": T")).is_some();

        let path_str = format!("{}/{}.rs", output_dir, &struct_filename);
        let path = PathBuf::from(&path_str);
        let Ok(file) = File::create(path) else {
            panic!("Failed to create file {}", &path_str);
        };

        let mut writer = BufWriter::new(&file);

        writeln!(&mut writer, "use crate::token::{{Token, TokenLiteral}};")?;
        writeln!(&mut writer, "use super::expr::{};", base_name)?;
        writeln!(&mut writer)?;
        writeln!(
            &mut writer,
            "{}",
            if has_t {
                format!("pub struct {struct_name}{base_name}<T> where T: {base_name},")
            } else {
                format!("pub struct {struct_name}{base_name}")
            }
        )?;
        writeln!(&mut writer, "{{")?;

        let mut field_names: Vec<&str> = Vec::new();
        for field in fields.iter() {
            writeln!(&mut writer, "\t{},", field)?;
            let Some(f_parts) = field.split_once(":") else {
                continue;
            };
            field_names.push(f_parts.0.trim());
        }
        writeln!(&mut writer, "}}")?;
        writeln!(&mut writer)?;

        writeln!(
            &mut writer,
            "{}",
            if has_t {
                format!("impl<T> {struct_name}{base_name}<T> where T: {base_name},")
            } else {
                format!("impl {struct_name}{base_name}")
            }
        )?;
        writeln!(&mut writer, "{{")?;

        writeln!(&mut writer, "\tpub fn new({field_str}) -> Self {{")?;

        write!(&mut writer, "\t\t{struct_name}{base_name} {{")?;
        write!(&mut writer, "{}", field_names.join(", "))?;
        writeln!(&mut writer, "}}")?;

        writeln!(&mut writer, "\t}}")?; // close new fn

        writeln!(&mut writer, "}}")?; // close impl

        writeln!(&mut writer)?;

        writeln!(
            &mut writer,
            "{}",
            if has_t {
                format!("impl <T> {base_name} for {struct_name}{base_name}<T> where T: {base_name} {{")
            } else {
                format!("impl {base_name} for {struct_name}{base_name} {{")
            }
        )?;
        writeln!(&mut writer, "}}")?;

        writer.flush()?;
    }

    // open the mod.rs file and append the new modules
    let path_str = format!("{}/mod.rs", output_dir);
    let path = PathBuf::from(&path_str);
    let Ok(file) = File::create(path) else  {
            panic!("Failed to open mod.rs file {}", &path_str);
        };

    println!("Mod declarations: {:?}", mod_declarations);
    let mut writer = BufWriter::new(&file);
    writeln!(&mut writer, "pub mod expr;")?;
    for decl in mod_declarations.iter() {
        writeln!(&mut writer, "pub mod {decl};")?;
    }

    Ok(())
}
