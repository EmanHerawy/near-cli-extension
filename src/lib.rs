use std::

{
    env,
    fs,
    io::{
        Cursor,
        Read,
        Seek,
        Write,
    },
    path::{
        Path,
        PathBuf,
    },
};
use std::error::Error;
use anyhow::Result;
use inquire::Select;
use inquire::Text;
   
pub struct Template{
    name: String,
    path: String,
    
}

impl Template{
    pub fn get_full_path(&self) -> PathBuf{
     let current_dir=env::current_dir().unwrap();
    match self.name.as_str(){
        "empty"=>current_dir.join("templates/empty"),
        "Hello World"=>current_dir.join("templates/hello_world"),
        "NFT"=>current_dir.join("templates/NFT"),
        _=>current_dir.join("templates/empty")}
    }
    pub fn new(name:String)->Self{
        Template{
            name,
            path:String::from("templates"),
        }
    }
}

pub fn run ()->Result<(),Box<dyn Error>>{
    let options =vec!["empty","Hello World","NFT"];

    let project_name = Text::new("Project Name")
    .with_help_message("Enter your project name")
    .with_default("hello world")
    .prompt()?;
let directory= Text::new("Directory")
    .with_help_message("Enter your project directory")
    .with_default("./")
    .prompt()?;
let template_name= Select::new("Please select a template",options)
    .with_help_message("Select a template")
    .prompt()?;


    let template = Template::new(String::from(template_name));
    let template_path=template.get_full_path();
    println!("template path is : {}", template_path.display());
        copy_dir_all(template_path, directory)?;
        Ok(())
}
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
        for file in fs::read_dir(&src)?{
        println!("file listed is : {}", file.unwrap().path().display());
    }
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
             fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            //     if let Some(name) = name {
            //     let mut contents = String::new();
            //     fs::File::open(&file)?.read_to_string(&mut contents)?;
            //     let contents = contents.replace("{{name}}", name);
            //     let contents = contents.replace("{{camel_name}}", &name.to_upper_camel_case());
            //     outfile.write_all(contents.as_bytes())?;
            // } else {
            //     let mut contents = Vec::new();
            //     fs::File::open(&file)?.read_to_end(&mut contents)?;
            //     outfile.write_all(&contents)?;
            // }
        }
    }
    Ok(())
}