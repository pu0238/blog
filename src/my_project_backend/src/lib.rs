use std::cell::RefCell;

use crate::blog::Blog;
use crate::config::Config;

mod blog;
mod config;

thread_local! {
    static CONFIG: RefCell<Config> = RefCell::new(Config::new());
    static BLOGS: RefCell<Vec<Blog>> = RefCell::new(Vec::new());
}
// komentarze ?

#[ic_cdk::update]
fn add_blog(title: String, content: String, tags: Vec<String>) -> Result<Blog, String>{
    let config
    if title.len() > 250 {
        return Err("Title is too long!".to_string())
    }
    if content.len() > 2000 {
        return Err("Content is too long!".to_string())
    }
    if tags.len() > 3 {
        return Err("Too many tags!".to_string())
    }
    let blog = Blog::new(title, content, tags);
    BLOGS.with(|blogs| blogs.borrow_mut().push(blog));
    let last_blog = BLOGS.with(|blogs| 
        blogs
        .borrow()
        .last()
        .expect("Vec should not be empty").clone());
    Ok(last_blog)
}

#[ic_cdk::query]
fn get_blogs() -> Vec<Blog> {
    BLOGS.with(|blogs| blogs.borrow().clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
