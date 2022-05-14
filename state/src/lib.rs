use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, FnArg, ItemFn};

/// This can be used to add a state parameter to any method of the following form:
/// ```rust
/// fn render(/* some self variant */, ctx: UpdateContext) -> /* any */;
/// ```
/// into:
/// ```rust
/// #[sphinx::use_state]
/// fn render(/* some self variant */, ctx: UpdateContext, state: State) -> /* any */;
/// ```
///
/// Note that the type given to the `state` parameter **must** be the same type as the state instance given to `sphinx::run_with_state`.
/// If it is not, a runtime panic will occur (a panic will also occur if the application was started with `sphinx::run`).
#[proc_macro_attribute]
pub fn use_state(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(input as ItemFn);

    match f.sig.inputs.len() {
        2 => panic!("missing state argument: #[use_state] adds a second function argument which is given the application state (type must match)"),
        3 => (),
        _ => return f.into_token_stream().into(),
    };

    // Last element should be state
    let state = f.sig.inputs.pop().unwrap();

    // Second last (now last) element should be context
    let ctx = f.sig.inputs.last().unwrap();

    // Determine state variable and type
    let (state_v, state_t) = match state.value() {
        FnArg::Typed(ref ty) => (&ty.pat, &ty.ty),
        FnArg::Receiver(_) => unreachable!(),
    };

    // Determine context variable
    let ctx_v = match ctx {
        FnArg::Typed(ty) => &ty.pat,
        FnArg::Receiver(_) => unreachable!(),
    };

    // Downcast internal state type to expected value
    let binding = quote! {
        let #state_v = #ctx_v.state.downcast_mut::<#state_t>().expect("incorrect state type");
    };

    // Prepend method with state binding
    let stmt = syn::parse(binding.into()).unwrap();
    f.block.stmts.insert(0, stmt);

    f.into_token_stream().into()
}
