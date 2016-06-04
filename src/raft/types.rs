// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct RequestVote {
    // message fields
    vote: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestVote {}

impl RequestVote {
    pub fn new() -> RequestVote {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestVote {
        static mut instance: ::protobuf::lazy::Lazy<RequestVote> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestVote,
        };
        unsafe {
            instance.get(|| {
                RequestVote {
                    vote: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 vote = 1;

    pub fn clear_vote(&mut self) {
        self.vote = ::std::option::Option::None;
    }

    pub fn has_vote(&self) -> bool {
        self.vote.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vote(&mut self, v: i32) {
        self.vote = ::std::option::Option::Some(v);
    }

    pub fn get_vote(&self) -> i32 {
        self.vote.unwrap_or(0)
    }
}

impl ::protobuf::Message for RequestVote {
    fn is_initialized(&self) -> bool {
        if self.vote.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.vote = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.vote.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.vote {
            try!(os.write_int32(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RequestVote>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestVote {
    fn new() -> RequestVote {
        RequestVote::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestVote>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "vote",
                    RequestVote::has_vote,
                    RequestVote::get_vote,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestVote>(
                    "RequestVote",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestVote {
    fn clear(&mut self) {
        self.clear_vote();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RequestVote {
    fn eq(&self, other: &RequestVote) -> bool {
        self.vote == other.vote &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RequestVote {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AppendEntry {
    // message fields
    entries: ::protobuf::RepeatedField<AppendEntry_Entry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AppendEntry {}

impl AppendEntry {
    pub fn new() -> AppendEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AppendEntry {
        static mut instance: ::protobuf::lazy::Lazy<AppendEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AppendEntry,
        };
        unsafe {
            instance.get(|| {
                AppendEntry {
                    entries: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .AppendEntry.Entry entries = 1;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<AppendEntry_Entry>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<AppendEntry_Entry> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<AppendEntry_Entry> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[AppendEntry_Entry] {
        &self.entries
    }
}

impl ::protobuf::Message for AppendEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.entries.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.entries.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AppendEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AppendEntry {
    fn new() -> AppendEntry {
        AppendEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<AppendEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "entries",
                    AppendEntry::get_entries,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AppendEntry>(
                    "AppendEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AppendEntry {
    fn clear(&mut self) {
        self.clear_entries();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AppendEntry {
    fn eq(&self, other: &AppendEntry) -> bool {
        self.entries == other.entries &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AppendEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AppendEntry_Entry {
    // message fields
    hash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    bytes: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AppendEntry_Entry {}

impl AppendEntry_Entry {
    pub fn new() -> AppendEntry_Entry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AppendEntry_Entry {
        static mut instance: ::protobuf::lazy::Lazy<AppendEntry_Entry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AppendEntry_Entry,
        };
        unsafe {
            instance.get(|| {
                AppendEntry_Entry {
                    hash: ::protobuf::SingularField::none(),
                    bytes: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes hash = 1;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.hash.is_none() {
            self.hash.set_default();
        };
        self.hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        self.hash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        match self.hash.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes bytes = 2;

    pub fn clear_bytes(&mut self) {
        self.bytes.clear();
    }

    pub fn has_bytes(&self) -> bool {
        self.bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.bytes = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bytes(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bytes.is_none() {
            self.bytes.set_default();
        };
        self.bytes.as_mut().unwrap()
    }

    // Take field
    pub fn take_bytes(&mut self) -> ::std::vec::Vec<u8> {
        self.bytes.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bytes(&self) -> &[u8] {
        match self.bytes.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for AppendEntry_Entry {
    fn is_initialized(&self) -> bool {
        if self.hash.is_none() {
            return false;
        };
        if self.bytes.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.hash));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bytes));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.hash.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.bytes.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hash.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.bytes.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AppendEntry_Entry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AppendEntry_Entry {
    fn new() -> AppendEntry_Entry {
        AppendEntry_Entry::new()
    }

    fn descriptor_static(_: ::std::option::Option<AppendEntry_Entry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "hash",
                    AppendEntry_Entry::has_hash,
                    AppendEntry_Entry::get_hash,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bytes",
                    AppendEntry_Entry::has_bytes,
                    AppendEntry_Entry::get_bytes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AppendEntry_Entry>(
                    "AppendEntry_Entry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AppendEntry_Entry {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_bytes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AppendEntry_Entry {
    fn eq(&self, other: &AppendEntry_Entry) -> bool {
        self.hash == other.hash &&
        self.bytes == other.bytes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AppendEntry_Entry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Snapshot {
    // message fields
    bytes: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Snapshot {}

impl Snapshot {
    pub fn new() -> Snapshot {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Snapshot {
        static mut instance: ::protobuf::lazy::Lazy<Snapshot> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Snapshot,
        };
        unsafe {
            instance.get(|| {
                Snapshot {
                    bytes: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes bytes = 1;

    pub fn clear_bytes(&mut self) {
        self.bytes.clear();
    }

    pub fn has_bytes(&self) -> bool {
        self.bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.bytes = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bytes(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bytes.is_none() {
            self.bytes.set_default();
        };
        self.bytes.as_mut().unwrap()
    }

    // Take field
    pub fn take_bytes(&mut self) -> ::std::vec::Vec<u8> {
        self.bytes.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bytes(&self) -> &[u8] {
        match self.bytes.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for Snapshot {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bytes));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.bytes.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bytes.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Snapshot>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Snapshot {
    fn new() -> Snapshot {
        Snapshot::new()
    }

    fn descriptor_static(_: ::std::option::Option<Snapshot>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bytes",
                    Snapshot::has_bytes,
                    Snapshot::get_bytes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Snapshot>(
                    "Snapshot",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Snapshot {
    fn clear(&mut self) {
        self.clear_bytes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Snapshot {
    fn eq(&self, other: &Snapshot) -> bool {
        self.bytes == other.bytes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Snapshot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RaftRPC {
    // message oneof groups
    msg: ::std::option::Option<RaftRPC_oneof_msg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RaftRPC {}

#[derive(Clone,PartialEq)]
pub enum RaftRPC_oneof_msg {
    request_vote(RequestVote),
    append_entry(AppendEntry),
    snapshot(Snapshot),
}

impl RaftRPC {
    pub fn new() -> RaftRPC {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftRPC {
        static mut instance: ::protobuf::lazy::Lazy<RaftRPC> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftRPC,
        };
        unsafe {
            instance.get(|| {
                RaftRPC {
                    msg: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .RequestVote request_vote = 1;

    pub fn clear_request_vote(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_request_vote(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(RaftRPC_oneof_msg::request_vote(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_request_vote(&mut self, v: RequestVote) {
        self.msg = ::std::option::Option::Some(RaftRPC_oneof_msg::request_vote(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request_vote(&mut self) -> &mut RequestVote {
        if let ::std::option::Option::Some(RaftRPC_oneof_msg::request_vote(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(RaftRPC_oneof_msg::request_vote(RequestVote::new()));
        }
        match self.msg {
            ::std::option::Option::Some(RaftRPC_oneof_msg::request_vote(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_request_vote(&mut self) -> RequestVote {
        if self.has_request_vote() {
            match self.msg.take() {
                ::std::option::Option::Some(RaftRPC_oneof_msg::request_vote(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestVote::new()
        }
    }

    pub fn get_request_vote(&self) -> &RequestVote {
        match self.msg {
            ::std::option::Option::Some(RaftRPC_oneof_msg::request_vote(ref v)) => v,
            _ => RequestVote::default_instance(),
        }
    }

    // optional .AppendEntry append_entry = 2;

    pub fn clear_append_entry(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_append_entry(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(RaftRPC_oneof_msg::append_entry(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_append_entry(&mut self, v: AppendEntry) {
        self.msg = ::std::option::Option::Some(RaftRPC_oneof_msg::append_entry(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_append_entry(&mut self) -> &mut AppendEntry {
        if let ::std::option::Option::Some(RaftRPC_oneof_msg::append_entry(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(RaftRPC_oneof_msg::append_entry(AppendEntry::new()));
        }
        match self.msg {
            ::std::option::Option::Some(RaftRPC_oneof_msg::append_entry(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_append_entry(&mut self) -> AppendEntry {
        if self.has_append_entry() {
            match self.msg.take() {
                ::std::option::Option::Some(RaftRPC_oneof_msg::append_entry(v)) => v,
                _ => panic!(),
            }
        } else {
            AppendEntry::new()
        }
    }

    pub fn get_append_entry(&self) -> &AppendEntry {
        match self.msg {
            ::std::option::Option::Some(RaftRPC_oneof_msg::append_entry(ref v)) => v,
            _ => AppendEntry::default_instance(),
        }
    }

    // optional .Snapshot snapshot = 3;

    pub fn clear_snapshot(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_snapshot(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(RaftRPC_oneof_msg::snapshot(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_snapshot(&mut self, v: Snapshot) {
        self.msg = ::std::option::Option::Some(RaftRPC_oneof_msg::snapshot(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snapshot(&mut self) -> &mut Snapshot {
        if let ::std::option::Option::Some(RaftRPC_oneof_msg::snapshot(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(RaftRPC_oneof_msg::snapshot(Snapshot::new()));
        }
        match self.msg {
            ::std::option::Option::Some(RaftRPC_oneof_msg::snapshot(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_snapshot(&mut self) -> Snapshot {
        if self.has_snapshot() {
            match self.msg.take() {
                ::std::option::Option::Some(RaftRPC_oneof_msg::snapshot(v)) => v,
                _ => panic!(),
            }
        } else {
            Snapshot::new()
        }
    }

    pub fn get_snapshot(&self) -> &Snapshot {
        match self.msg {
            ::std::option::Option::Some(RaftRPC_oneof_msg::snapshot(ref v)) => v,
            _ => Snapshot::default_instance(),
        }
    }
}

impl ::protobuf::Message for RaftRPC {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(RaftRPC_oneof_msg::request_vote(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(RaftRPC_oneof_msg::append_entry(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(RaftRPC_oneof_msg::snapshot(try!(is.read_message())));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &RaftRPC_oneof_msg::request_vote(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RaftRPC_oneof_msg::append_entry(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RaftRPC_oneof_msg::snapshot(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &RaftRPC_oneof_msg::request_vote(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &RaftRPC_oneof_msg::append_entry(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &RaftRPC_oneof_msg::snapshot(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RaftRPC>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RaftRPC {
    fn new() -> RaftRPC {
        RaftRPC::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftRPC>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "request_vote",
                    RaftRPC::has_request_vote,
                    RaftRPC::get_request_vote,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "append_entry",
                    RaftRPC::has_append_entry,
                    RaftRPC::get_append_entry,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "snapshot",
                    RaftRPC::has_snapshot,
                    RaftRPC::get_snapshot,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaftRPC>(
                    "RaftRPC",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftRPC {
    fn clear(&mut self) {
        self.clear_request_vote();
        self.clear_append_entry();
        self.clear_snapshot();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RaftRPC {
    fn eq(&self, other: &RaftRPC) -> bool {
        self.msg == other.msg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RaftRPC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0f, 0x73, 0x72, 0x63, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0x1b, 0x0a, 0x0b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x56, 0x6f, 0x74, 0x65,
    0x12, 0x0c, 0x0a, 0x04, 0x76, 0x6f, 0x74, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05, 0x22, 0x58,
    0x0a, 0x0b, 0x41, 0x70, 0x70, 0x65, 0x6e, 0x64, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x23, 0x0a,
    0x07, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x12,
    0x2e, 0x41, 0x70, 0x70, 0x65, 0x6e, 0x64, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x45, 0x6e, 0x74,
    0x72, 0x79, 0x1a, 0x24, 0x0a, 0x05, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x0c, 0x0a, 0x04, 0x68,
    0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0d, 0x0a, 0x05, 0x62, 0x79, 0x74,
    0x65, 0x73, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x22, 0x19, 0x0a, 0x08, 0x53, 0x6e, 0x61, 0x70,
    0x73, 0x68, 0x6f, 0x74, 0x12, 0x0d, 0x0a, 0x05, 0x62, 0x79, 0x74, 0x65, 0x73, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0c, 0x22, 0x7b, 0x0a, 0x07, 0x52, 0x61, 0x66, 0x74, 0x52, 0x50, 0x43, 0x12, 0x24,
    0x0a, 0x0c, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x76, 0x6f, 0x74, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x56, 0x6f,
    0x74, 0x65, 0x48, 0x00, 0x12, 0x24, 0x0a, 0x0c, 0x61, 0x70, 0x70, 0x65, 0x6e, 0x64, 0x5f, 0x65,
    0x6e, 0x74, 0x72, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x41, 0x70, 0x70,
    0x65, 0x6e, 0x64, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x48, 0x00, 0x12, 0x1d, 0x0a, 0x08, 0x73, 0x6e,
    0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x53,
    0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x48, 0x00, 0x42, 0x05, 0x0a, 0x03, 0x6d, 0x73, 0x67,
    0x4a, 0xb2, 0x05, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x00, 0x00, 0x02, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x00, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x01, 0x02, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x01, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x01, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x01, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x01, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x04, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x04, 0x08, 0x13,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x03, 0x00, 0x12, 0x04, 0x05, 0x02, 0x08, 0x03, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x01, 0x12, 0x03, 0x05, 0x0a, 0x0f, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x04, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x06, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x06, 0x0d, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x13, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x06, 0x1a, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x07, 0x04, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x07, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x07, 0x0d, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07, 0x13, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x07, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x09, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x11,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x1b, 0x1c, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0c, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x0d, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0d,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x10, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x10, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x08, 0x00, 0x12, 0x04, 0x11, 0x02,
    0x15, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x08, 0x00, 0x01, 0x12, 0x03, 0x11, 0x08, 0x0b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x12, 0x04, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x12, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x12, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12,
    0x03, 0x13, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x13,
    0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x13, 0x10, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x13, 0x1f, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x14, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x06, 0x12, 0x03, 0x14, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x14, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x14, 0x18, 0x19,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
