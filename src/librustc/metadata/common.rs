// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


// EBML enum definitions and utils shared by the encoder and decoder

pub const tag_items: uint = 0x02u;

pub const tag_paths_data_name: uint = 0x04u;

pub const tag_def_id: uint = 0x07u;

pub const tag_items_data: uint = 0x08u;

pub const tag_items_data_item: uint = 0x09u;

pub const tag_items_data_item_family: uint = 0x0au;

pub const tag_items_data_item_ty_param_bounds: uint = 0x0bu;

pub const tag_items_data_item_type: uint = 0x0cu;

pub const tag_items_data_item_symbol: uint = 0x0du;

pub const tag_items_data_item_variant: uint = 0x0eu;

pub const tag_items_data_parent_item: uint = 0x0fu;

pub const tag_index: uint = 0x11u;

pub const tag_index_buckets: uint = 0x12u;

pub const tag_index_buckets_bucket: uint = 0x13u;

pub const tag_index_buckets_bucket_elt: uint = 0x14u;

pub const tag_index_table: uint = 0x15u;

pub const tag_meta_item_name_value: uint = 0x18u;

pub const tag_meta_item_name: uint = 0x19u;

pub const tag_meta_item_value: uint = 0x20u;

pub const tag_attributes: uint = 0x21u;

pub const tag_attribute: uint = 0x22u;

pub const tag_meta_item_word: uint = 0x23u;

pub const tag_meta_item_list: uint = 0x24u;

// The list of crates that this crate depends on
pub const tag_crate_deps: uint = 0x25u;

// A single crate dependency
pub const tag_crate_dep: uint = 0x26u;

pub const tag_crate_hash: uint = 0x28u;

pub const tag_parent_item: uint = 0x29u;

pub const tag_crate_dep_name: uint = 0x2au;
pub const tag_crate_dep_hash: uint = 0x2bu;
pub const tag_crate_dep_vers: uint = 0x2cu;

pub const tag_mod_impl: uint = 0x30u;

pub const tag_item_trait_method: uint = 0x31u;
pub const tag_impl_trait: uint = 0x32u;

// discriminator value for variants
pub const tag_disr_val: uint = 0x34u;

// used to encode ast_map::path and ast_map::path_elt
pub const tag_path: uint = 0x40u;
pub const tag_path_len: uint = 0x41u;
pub const tag_path_elt_mod: uint = 0x42u;
pub const tag_path_elt_name: uint = 0x43u;
pub const tag_item_field: uint = 0x44u;
pub const tag_struct_mut: uint = 0x45u;

pub const tag_region_param: uint = 0x46u;
pub const tag_mod_impl_trait: uint = 0x47u;
/*
  trait items contain tag_item_trait_method elements,
  impl items contain tag_item_impl_method elements, and classes
  have both. That's because some code treats classes like traits,
  and other code treats them like impls. Because classes can contain
  both, tag_item_trait_method and tag_item_impl_method have to be two
  different tags.
 */
pub const tag_item_impl_method: uint = 0x48u;
pub const tag_item_dtor: uint = 0x49u;
pub const tag_item_trait_method_self_ty: uint = 0x4b;
pub const tag_item_trait_method_self_ty_region: uint = 0x4c;

// Reexports are found within module tags. Each reexport contains def_ids
// and names.
pub const tag_items_data_item_reexport: uint = 0x4d;
pub const tag_items_data_item_reexport_def_id: uint = 0x4e;
pub const tag_items_data_item_reexport_name: uint = 0x4f;

// used to encode crate_ctxt side tables
pub enum astencode_tag { // Reserves 0x50 -- 0x6f
    tag_ast = 0x50,

    tag_tree = 0x51,

    tag_id_range = 0x52,

    tag_table = 0x53,
    tag_table_id = 0x54,
    tag_table_val = 0x55,
    tag_table_def = 0x56,
    tag_table_node_type = 0x57,
    tag_table_node_type_subst = 0x58,
    tag_table_freevars = 0x59,
    tag_table_tcache = 0x5a,
    tag_table_param_bounds = 0x5b,
    tag_table_inferred_modes = 0x5c,
    tag_table_mutbl = 0x5d,
    tag_table_last_use = 0x5e,
    tag_table_spill = 0x5f,
    tag_table_method_map = 0x60,
    tag_table_vtable_map = 0x61,
    tag_table_adjustments = 0x62,
    tag_table_moves_map = 0x63,
    tag_table_capture_map = 0x64
}

pub const tag_item_trait_method_sort: uint = 0x70;

pub const tag_item_impl_type_basename: uint = 0x71;

// Language items are a top-level directory (for speed). Hierarchy:
//
// tag_lang_items
// - tag_lang_items_item
//   - tag_lang_items_item_id: u32
//   - tag_lang_items_item_node_id: u32

pub const tag_lang_items: uint = 0x72;
pub const tag_lang_items_item: uint = 0x73;
pub const tag_lang_items_item_id: uint = 0x74;
pub const tag_lang_items_item_node_id: uint = 0x75;

pub const tag_item_unnamed_field: uint = 0x76;
pub const tag_items_data_item_struct_ctor: uint = 0x77;
pub const tag_items_data_item_visibility: uint = 0x78;

pub struct LinkMeta {
    name: @str,
    vers: @str,
    extras_hash: @str
}

