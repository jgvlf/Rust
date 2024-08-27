mod modules;

fn main() {
    // modules::guessing_game::main::guessing_game_run();
    println!("code execute in \"main()\" function");
    modules::three::functions::another_function(5);
    modules::three::functions::print_label_meansurement(900, 'W')
}
