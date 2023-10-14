use std::{
    env,
    fs,
    io::{
        Read,
        Seek,
        Write,
    },
    path::{
        Path,
        PathBuf,
    }, borrow::Cow,
};
use std::error::Error;
use anyhow::Result;
use inquire::Select;
use inquire::Text;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "templates"]
#[prefix = "assets/"]
struct Assets;


// #[derive(RustEmbed)]
// #[folder = "templates/empty"]
// #[prefix = "empty/"]
// struct Empty;
// #[derive(RustEmbed)]
// #[folder = "templates/nft"]
// #[prefix = "nft/"]
// struct NFT;
// #[derive(RustEmbed)]
// #[folder = "templates/storage"]
// #[prefix = "storage/"]
// struct Storage;
// #[derive(RustEmbed)]
// #[folder = "templates/hello_world"]
// #[prefix = "hello/"]
// struct HelloWorld;

fn get_sub_folder<'a>(path: &'a str) -> impl Iterator<Item = Cow<'a, str>> + 'a {
    Assets::iter()
        .filter(move|file| file.as_ref().starts_with(path))
        .map(move|file| file.as_ref().replace(path, "").into())
}
// enum TemplateType {
//     Empty(Empty),
//     NFT(NFT),
//     Storage(Storage),
//     HelloWorld(HelloWorld),
// }

 

// impl AssetTrait for TemplateType {
//     fn get_subfolder(&self) -> std::borrow::Cow<'static, str> {
//         match self {
//             TemplateType::Empty(_) => "empty/".into(),
//             TemplateType::NFT(_) => "nft/".into(),
//             TemplateType::Storage(_) => "storage/".into(),
//             TemplateType::HelloWorld(_) => "hello/".into(),
//         }
//     }
// }

pub fn run() -> Result<(), Box<dyn Error>> {
    let options = vec!["empty", "Hello World", "NFT","Storage"];
    
    let project_name = Text::new("Project Name")
        .with_help_message("Enter your project name")
        .with_default("hello_world")
        .prompt()?;
    let directory = Text::new("Directory")
        .with_help_message("Enter your project directory")
        .with_default("./")
        .prompt()?;
    let template_name = Select::new("Please select a template", options)
        .with_help_message("Select a template")
        .prompt()?;
    let template = match template_name {
        "empty" => get_sub_folder("assets/empty"),
        "Storage" => get_sub_folder("assets/storage"),
        "Hello World" => get_sub_folder("assets/hello"),
        "NFT" => get_sub_folder("assets/nft"),
        _ => get_sub_folder("assets/empty"),
      
    };


    // copy_dir_all(template, directory, project_name.as_str())?;
    Ok(())
}



pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>, name: &str) -> Result<()> {
    for file in fs::read_dir(&src)? {
        println!("file listed is : {}", file.unwrap().path().display());
    }

    let dst = dst.as_ref();

    // Check if the destination directory already exists
    if dst.exists() && !dst.is_dir() {
        // Handle the case where dst is a file, not a directory
        return Err(anyhow::anyhow!("Destination is not a directory: {}", dst.display()));
    }

    // Create the destination directory if it doesn't exist
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.join(entry.file_name()), name)?;
        } else if ty.is_file() {
            let dest_file = dst.join(entry.file_name());

            // Check if the destination file already exists
            if dest_file.exists() {
                // Handle the case where the file already exists, e.g., prompt the user for confirmation
                println!("File already exists: {}", dest_file.display());
                // You can add logic here to prompt the user for confirmation or decide how to handle this case
            } else {
                fs::copy(entry.path(), &dest_file)?;

                // replace name = "{{name}}" with name = "project name" in Cargo.toml
                if entry.file_name().to_str().unwrap() == "Cargo.toml" {
                    let mut file = fs::OpenOptions::new()
                        .read(true)
                        .write(true)
                        .open(dest_file)?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?;
                    let contents = contents.replace("{{name}}", name);
                    file.seek(std::io::SeekFrom::Start(0))?;
                    file.write_all(contents.as_bytes())?;
                    file.set_len(contents.len() as u64)?;
                }
            }
        }
    }

    Ok(())
}
