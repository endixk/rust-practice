extern crate blog;
use blog::{Post, PostType};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    post.request_review();
    post.approve();
    post.approve();

    println!("Post: {}", post.content());

    let mut post = PostType::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    let mut post = post.reject();
    post.add_text(" and I ate a salad for dinner too");
    println!("Post: {}", post.request_review().approve().approve().content());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hide_content_before_publish(){
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

    #[test]
    fn reject_post(){
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        post.request_review();
        post.reject();
        assert_eq!("", post.content());

        post.approve();
        post.approve();
        assert_eq!("", post.content());

        post.request_review();
        post.approve();
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

    #[test]
    fn approve_once(){
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        post.request_review();
        post.approve();
        assert_eq!("", post.content());

        post.reject();
        post.request_review();
        post.approve();
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

    #[test]
    fn edit_during_approval(){
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        post.add_text(" and a sandwich for dinner");
        post.request_review();
        post.add_text("I ate a salad for lunch today");
        post.approve();
        post.approve();
        assert_eq!("I ate a salad for lunch today and a sandwich for dinner", post.content());
    }
}