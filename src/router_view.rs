use std::rc::Rc;

use crate::node::Node;

pub struct RouterView {
    router: Router,
}

impl RouterView {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    pub fn route(
        &mut self,
        default_route_path: &str,
        routes: Vec<RouterPath>,
    ) -> Result<(), String> {
        let mut final_routes = vec![];
        let mut default_route = None;
        for path in routes {
            let is_default_path = path.path == default_route_path;
            let new_path = Rc::new(path);
            if is_default_path {
                default_route = Some(Rc::clone(&new_path));
            }
            final_routes.push(new_path);
        }
        if let Some(value) = default_route {
            self.router = Router {
                routes: final_routes,
                default_route: Some(value),
            };
            Ok(())
        } else {
            Err("Invalid default route path".to_string())
        }
    }
}

struct Router {
    pub routes: Vec<Rc<RouterPath>>,
    pub default_route: Option<Rc<RouterPath>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: Default::default(),
            default_route: None,
        }
    }
}

pub struct RouterPath {
    pub path: String,
    pub content: Box<dyn Fn() -> Node>,
}

impl RouterPath {
    pub fn new(path: &str, content_callback: impl Fn() -> Node + 'static) -> Self {
        Self {
            path: path.to_string(),
            content: Box::new(content_callback),
        }
    }
}
