use std::rc::Rc;

pub struct Context {
    pub router: Option<Router>,
}

impl Context {
    pub fn route(
        &mut self,
        default_route_path: &str,
        routes: Vec<RouterPath>,
    ) -> Result<(), String> {
        if self.router.is_none() {
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
                self.router = Some(Router {
                    routes: final_routes,
                    default_route: value,
                });
                Ok(())
            } else {
                Err("Invalid default route path".to_string())
            }
        } else {
            Err("Cannot call this function more than once".to_string())
        }
    }
}

pub struct Router {
    pub routes: Vec<Rc<RouterPath>>,
    pub default_route: Rc<RouterPath>,
}

pub struct RouterPath {
    pub path: String,
}

impl RouterPath {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}
