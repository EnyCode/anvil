use web::App;

mod anvil;
mod enchantments;
mod item;
mod web;

fn main() {
    yew::Renderer::<App>::new().render();
}
