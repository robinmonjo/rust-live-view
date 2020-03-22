use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    counter_component: String,
}

#[derive(Template)]
#[template(path = "counter.html")]
struct CounterTemplate {
    count: i32,
}

pub fn counter_component(count: i32) -> String {
    CounterTemplate { count }.render().unwrap()
}

pub fn index_component(count: i32) -> String {
    let counter_component = counter_component(count);
    IndexTemplate { counter_component }.render().unwrap()
}
