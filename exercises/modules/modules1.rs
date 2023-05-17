// modules1.rs
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a hint.

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // pub: 表示该项是公共的，可以从任何地方访问。
    // crate: 表示可见性的作用域被限制在当前 crate 内。
    pub(crate) fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
