mod let_mut_sample;
mod string_array_vector_sample;
mod tupple_sample;
mod if_sample;
mod for_while_loop;
mod match_sample;
mod fn_closure_sample;
mod struct_sample;
mod enum_sample;
mod option_result_sample;
mod ownership_sample;
mod ownership_with_lifetime_param_sample;
mod trait_sample;
mod async_await_sample;

fn main() {
    let_mut_sample::example();
    string_array_vector_sample::example();
    tupple_sample::example();
    if_sample::example();
    for_while_loop::example();
    match_sample::example();
    fn_closure_sample::example();
    struct_sample::example();
    enum_sample::example();
    ownership_sample::example();
    ownership_with_lifetime_param_sample::example();
    trait_sample::example();
    async_await_sample::example();
}
