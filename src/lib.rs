use std::{
    fs,
    io::{
        Write,
    },
    path::{
        Path,
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

// fn get_sub_folder<'a>(path: &'a str) -> impl Iterator<Item = Cow<'a, str>> + 'a {
//     Assets::iter()
//         .filter(move|file| file.as_ref().starts_with(path))
//         .map(move|file| file.as_ref().replace(path, "").into())
// }
pub fn get_sub_folder(path: &str) -> Cow<'static, str> {
    let sub_folders: Vec<Cow<'static, str>> = Assets::iter()
        .filter(|file| file.as_ref().starts_with(path))
        .map(|file| file.as_ref().replace(path, "").into())
        .collect();
    sub_folders.join("\n").into()
}
struct Templates<'a> {
    name: &'a str,
    data: std::borrow::Cow<'static, str>,
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
        "empty" => Templates{name : "assets/empty", data:get_sub_folder("assets/empty")},
        "Hello World" => Templates{name : "assets/hello_world", data:get_sub_folder("assets/hello_world")},
        "NFT" => Templates{name : "assets/nft", data:get_sub_folder("assets/nft")},
        "Storage" => Templates{name : "assets/storage", data:get_sub_folder("assets/storage")},
        _ => Templates{name : "assets/empty", data:get_sub_folder("assets/empty")},
      
    };


    copy_template(template, directory, project_name.as_str())?;
    Ok(())
}


 fn copy_template(
    template:Templates,
    dst: impl AsRef<Path>,
    name: &str,
) -> Result<()> {
    let dst = dst.as_ref();
    // Create the destination directory if it doesn't exist
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

     for entry in template.data.lines() {
         let file_path = entry.to_string();
        let template_entry = Assets::get(format!("{}{}",&template.name, file_path).as_str()).unwrap();
        let template_entry_bytes: &[u8] = &template_entry.data;

        let dest_file = dst.join(format!("{}{}", &name,&file_path));
        println!("creating file : {}", dest_file.display());
        let parent_dir = dest_file.parent().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to determine parent directory")
        })?;

        //Create the parent directory if it doesn't exist
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }

        // Check if the destination file already exists
        if dest_file.exists() {
            // Handle the case where the file already exists, e.g., prompt the user for confirmation
            println!("File already exists: {}", dest_file.display());
            // You can add logic here to prompt the user for confirmation or decide how to handle this case
        } else {
            let mut dest_file = fs::File::create(&dest_file)?;
           if file_path == "/Cargo.toml" {
            // replace {{name}} with project name in the copied file
                    let mut content_str = String::from_utf8(template_entry_bytes.to_vec())?;
                    content_str = content_str.replace("{{name}}", name);
                    let content_bytes = content_str.as_bytes();
                     dest_file.write_all(content_bytes)?;

           }else{
             dest_file.write_all(template_entry_bytes)?;
           
            }
        }
    }
    Ok(())
}


