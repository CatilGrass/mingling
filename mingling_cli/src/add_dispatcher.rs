use mingling::{
    AnyOutput,
    macros::{chain, dispatcher, pack, r_println, renderer},
    marker::NextProcess,
    parser::Picker,
};

use crate::MinglingCLI;

dispatcher!(MinglingCLI, "add.dispatcher", AddDispatcherCommand => AddDispatcherEntry);

#[chain(MinglingCLI)]
pub async fn parse_add_dispatcher(args: AddDispatcherEntry) -> NextProcess {
    let picker: Picker<MinglingCLI> = Picker::new(args.inner);
    let dispatcher_name = picker.pick::<String>(()).unpack_directly().0;
    let input = AddDispatcherInput::new(dispatcher_name);
    input.into()
}

pack!(MinglingCLI, AddDispatcherInput = String);

#[chain(MinglingCLI)]
pub async fn exec_add_dispatcher(_input: AddDispatcherInput) -> NextProcess {
    AnyOutput::new(AddDispatcherSuccess::new(())).route_chain()
}

pack!(MinglingCLI, AddDispatcherSuccess = ());
pack!(MinglingCLI, AddDispatcherFailed = String);

#[renderer(MinglingCLI)]
pub fn render_add_dispatcher_success(_prev: AddDispatcherSuccess) {
    r_println!("Dispatcher added successfully");
}

#[renderer(MinglingCLI)]
pub fn render_add_dispatcher_failed(prev: AddDispatcherFailed) {
    r_println!("Error: {}", prev.inner);
}
