use mingling::{
    AnyOutput,
    macros::{chain, dispatcher, pack, r_println, renderer},
    marker::NextProcess,
    parser::Picker,
};

use crate::ThisProgram;

dispatcher!("add.dispatcher", AddDispatcherCommand => AddDispatcherEntry);
dispatcher!("remove.dispatcher", RemoveDispatcherCommand => RemoveDispatcherEntry);

#[chain]
pub fn parse_add_dispatcher(args: AddDispatcherEntry) -> NextProcess {
    let picker: Picker<ThisProgram> = Picker::new(args.inner);
    let dispatcher_name = picker.pick::<String>(()).unpack_directly().0;

    AddDispatcherInput::new(dispatcher_name)
}

#[chain]
pub fn parse_remove_dispatcher(args: RemoveDispatcherEntry) -> NextProcess {
    let picker: Picker<ThisProgram> = Picker::new(args.inner);
    let dispatcher_name = picker.pick::<String>(()).unpack_directly().0;

    AddDispatcherInput::new(dispatcher_name)
}

pack!(AddDispatcherInput = String);

#[chain]
pub fn exec_add_dispatcher(_input: AddDispatcherInput) -> NextProcess {
    AnyOutput::new(AddDispatcherSuccess::new(())).route_chain()
}

pack!(AddDispatcherSuccess = ());
pack!(AddDispatcherFailed = String);

#[renderer]
pub fn render_add_dispatcher_success(_prev: AddDispatcherSuccess) {
    r_println!("Dispatcher added successfully");
}

#[renderer]
pub fn render_add_dispatcher_failed(prev: AddDispatcherFailed) {
    r_println!("Error: {}", prev.inner);
}
