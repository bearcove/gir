use std::vec::Vec;

use env::Env;
use gobjects::*;
use library;

pub struct StatusedTypeId{
    pub type_id: library::TypeId,
    pub name: String,
    pub status: GStatus,
}

pub fn analyze_parents(env: &Env, type_id: library::TypeId) -> (Vec<StatusedTypeId>, bool) {
    let mut parents = Vec::new();
    let mut has_ignored_parents = false;
    let type_ = env.library.type_(type_id).to_class();

    for &parent_tid in &type_.parents {
        let parent_type = env.library.type_(parent_tid).to_class();

        let default_object: GObject = Default::default();
        let gobject = env.config.objects.get(&parent_tid.full_name(&env.library))
            .unwrap_or(&default_object);

        parents.push(StatusedTypeId{
            type_id: parent_tid,
            name: parent_type.name.clone(),
            status: gobject.status,
        });

        if gobject.status == GStatus::Ignore { has_ignored_parents = true; }

        if gobject.last_parent { break }
    }

    (parents, has_ignored_parents)
}