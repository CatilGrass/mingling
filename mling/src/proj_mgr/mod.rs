use mingling::macros::dispatcher;

dispatcher!("install");

dispatcher!("ls.namespace", CMDListNamespace => EntryListNamespace);
dispatcher!("rm.namespace", CMDRemoveNamespace => EntryRemoveNamespace);
