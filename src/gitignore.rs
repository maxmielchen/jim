use std::fs;
use std::io;
use regex::Regex;

use crate::handshake::Handshakable;

// CRUD

pub fn create(content: Option<String>) -> io::Result<fs::File>
{
    let file = fs::File::create(".gitignore");

    match file 
    {
        Err(e) => return Err(e),
        Ok(_) =>
        {
            if content != None
            {
                match io::Write::write_all(&mut file.as_ref().unwrap(), content.unwrap().as_bytes()) 
                {
                    Err(e) => return Err(e),
                    Ok(_) => {}
                }
            }
            return file
        }
    }
}

pub fn read() -> io::Result<String>
{
    let file = fs::File::open(".gitignore");

    match file 
    {
        Err(e) => return Err(e),
        Ok(_) =>
        {
            let mut content = String::new();
            match io::Read::read_to_string(&mut file.as_ref().unwrap(), &mut content)
            {
                Err(e) => return Err(e),
                Ok(_) => return Ok(content)
            }
        }
    }
}

pub fn update(content: &String) -> io::Result<fs::File>
{
    let file = fs::File::open(".gitignore");

    match file
    {
        Err(e) => return Err(e),
        Ok(_) =>
        {
            match io::Write::write_all(&mut file.as_ref().unwrap(), content.as_bytes()) 
            {
                Err(e) => return Err(e),
                Ok(_) => return file
            }
        }
    }
}

pub fn delete() -> io::Result<()>
{
    let file = fs::File::open(".gitignore");

    match file 
    {
        Err(e) => return Err(e),
        Ok(_) =>
        {
            fs::remove_file(".gitignore").unwrap();
            return Ok(())
        }
    }
}

// Trait

pub trait Engine
{
    fn fetch_gitignore(&self, templates: &Vec<String>) -> String;

    fn fetch_excludings(&self, templates: &Vec<String>) -> Vec<String>
    {
        let mut excludings: Vec<String> = Vec::new();
        for line in self.fetch_gitignore(templates).lines()
        {
            let simplified_line: String = String::from(line.trim());
            if simplified_line.is_empty() || simplified_line.starts_with("#") || excludings.contains(&simplified_line)
            {
                continue;
            }
            excludings.push(simplified_line);
        }
        return excludings;
    }

    fn fetch_clean_gitignore(&self, templates: &Vec<String>) -> String
    {
        let mut cleaned_gitignore: String = String::new();
        for exclude in self.fetch_excludings(&templates)
        {
            cleaned_gitignore.push_str(format!("{}\n", exclude.trim()).as_str())
        }
        return cleaned_gitignore;
    }
}

pub trait Sync
{
    fn get_templates(&self) -> Vec<String>;

    fn get_engine(&self) -> Box<dyn Engine>;

    fn define_head(&self) -> String 
    {
        let mut head = String::from("#");
        for template in self.get_templates()
        {
            head.push_str(format!(" {}", &template).as_str());
        } 
        return head;
    }

    fn define_body(&self) -> String
    {
        return self.get_engine().fetch_clean_gitignore(&self.get_templates());
    }

    fn fetch_templates(&self) -> Vec<String>
    {
        match read()
        {
            Err(_) => return Vec::new(),
            Ok(content) => 
            {
                let mut templates: Vec<String> = Vec::new();
                let unmanaged_templates: Vec<&str> = Regex::new(r#"(?<=\s|^)(?!#)\S+"#).unwrap().split(&content).filter(|w| !w.starts_with("#")).collect();
                for template in  unmanaged_templates
                {
                    if templates.contains(&String::from(template.trim()))
                    {
                        continue;
                    } 
                    templates.push(String::from(template.trim()))
                }
                return templates;
            }
        }
    }

    fn write(&self)
    {
        match update(&String::from(format!("{}\n{}", self.define_head(), self.define_body()).as_str())) 
        {
            Err(_) => 
            {
                create(
                    Some(String::from(format!("{}\n{}", self.define_head(), self.define_body()).as_str()))
                ).unwrap();
            },
            Ok(_) => {}
        }
    }
}

// Implementation

pub struct DefaultEngine 
{
    templates: Vec<String>
}

impl DefaultEngine 
{
    fn init() -> DefaultEngine
    {
        let mut engine = DefaultEngine 
        {
            templates: Vec::new(),
        };
        engine.templates = engine.fetch_templates();
        return engine;
    }
}

impl Engine for DefaultEngine 
{
    fn fetch_gitignore(&self, templates: &Vec<String>) -> String 
    {
        todo!()
    }
}

impl Sync for DefaultEngine
{
    fn get_templates(&self) -> Vec<String> 
    {
        self.templates.clone()
    }

    fn get_engine(&self) -> Box<dyn Engine> 
    {
        return Box::new(DefaultEngine{ templates: Vec::new() })
    }
}

impl Handshakable for DefaultEngine
{
    fn add(&mut self, names: Vec<String>) 
    {
        self.templates.extend(names.clone());
        self.write();
        println!("Successfully add:");
        for name in names
        {
            print!(" {}", name)
        }
    }

    fn list(&mut self, all: bool) 
    {
        if all
        { 
            for excluding in self.fetch_excludings(&self.templates) 
            {
                print!("{}\n", &excluding)
            }
            self.update();
        } else 
        {
            for template in self.get_templates() 
            {
                print!("{}\n", &template)
            }
            self.update();
        }
    } 

    fn delete(&mut self, all: bool, names: Vec<String>) 
    {
        if all
        { 
            delete().unwrap();
            println!("Successfully removed '.gitignore'");
        } else 
        {
            println!("Successfully removed:");
            for name in names
            {
                self.templates.retain(|ref d| d.as_str() == &name);
                print!(" {}", &name)
            }
            self.update();
        }
    }

    fn search(&self, query: String) 
    {
        todo!()
    }

    fn update(&mut self) 
    {
        self.write();
        println!("Successfully updated '.gitignore'");
    }
}
