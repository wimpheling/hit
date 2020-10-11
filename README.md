# 🎯 hit

![Build Status](https://github.com/wimpheling/hit/workflows/build/badge.svg)
![codecov](https://codecov.io/gh/wimpheling/hit/branch/master/graph/badge.svg?token=czzbPcCyZM)

`hit` is a Rust library to handle data structured in tree-like documents with these features:

- **H**ierarchical
- **I**ndexed
- **T**yped

This library was intended to manage, in memory, deeply nested documents with strictly typed data structures and multiple inner links. That could be the representation of a word processor document, a directory and its files and subfolders with symbolic links...

## Hierarchical

Every document is structured like a document tree, in a similar way to MongoDB documents. That means a document always start with a root `object`. In `hit` an `object` is defined as a key/value list.

The values can be either be

- simple (string, numeric, date)
- or they can contain **other sub objects**. Every sub-object (except the root one) can thus be located as being in a **property** of another **object**.
- they can also contain **references** to other objects.

(TODO : link to property types)

## Indexed

Every `object` <!-- (except, not yet implemented, embedded sub-objects) --> is indexed. That implies :

- every object must have an `_id`, with a rust `String` value
- every object (except the root object) can be located using these three indices :
  - parent_id
  - parent_property
  - parent_position

The indexation allows `hit` to provide (TODO LINK) `reference` and `reference_array` type fields. They are inspired by foreign keys in relation databases, and `hit` enforces consistency rules : you cannot delete an `object` as long as there are references to it in the document.

The index also allows you to easily find all the references to an object. (TODO: link to method)

## Typed

Every `object` in a document must have a (TODO: Link) `Model`. A model is identified by a string id, and is referenced in the `type` property of the `object`. To resolve model definitions from the ids, every instance of `hit` must be initialized with a (TODO: link) `Kernel` that contains the definitions.

The models :

- list the names of the accepted fields of an object
- restrict the accepted values using `field types` (TODO: link) and - optionally - `validators` (TODO : link)

# Get started

`hit` is a rust library. You can add it to your project by adding this line to your `Cargo.toml` file :

```
TODO
```

<!-- > We will sometimes use JSON representations of the `hit` documents. This is chosen for readability as it is a concise and well-known format, but JSON is not the native format of `hit`. A JSON serializer/deserializer is available, but the output we show here is not exactly the same as the JSON serializer output.
 -->

# Guide : How to create and use a `hit` instance

## Creating a `hit` instance

To create a `hit` data instance, you need a (TODO: link) `Kernel` with model definitions. One of the core kernels, officially supported by me, is the recursively designed (TODO: link) `hit_model`, which allows you to modelize models for `hit`.

To make it more simple, let's start with the basic, although completely useless (TODO: link) `hit_test_file_model`, that represents a directory/file structure, with links.

We will use ( TODO : link ) `Hit::new_with_values` to create the model. If you do not have initial values, you can instead use the (TODO : link ) `Hit::new` function.

```rust
use hit_file_model::create_kernel;
use hit::{ Hit, ObjectValue, IndexEntryProperty };
use std::collections::HashMap;

// create the kernel
let kernel = create_kernel();

// create a string id for the object
let id = "my_id".into();

// initiate the name value for our root object
let mut values = HashMap::new();
values.insert("name".into(), ObjectValue::String("name".into()));

// we can now create the hit instance
let hit_instance = Hit::new_with_values(
  id,
  kernel,
  values,
  // you must specify the main model name
  "file/filesystem"
);
```

## Property types

`hit` allows the following property types as values. The (TODO: link) `ObjectValue` enum handles this type system.

### Simple values

These values are set using the (TODO: link) `Hit::set` value.

- **string**

  a `string` field accepts rust `String` values.

- **number**

  a `number` field accepts rust `f32` values.

- **boolean**
- **date**

  a `date` field accepts timestamps as rust `i64` values.

### Complex values

These fields can only be populated using specific methods from the `Hit` struct.

- **sub_object**

  a `sub_object` field accepts a single `hit` object as a value. The field will be the only parent of the object that populates it.

- **sub_object_array**

  a `sub_object_array` field accepts several `hit` objects as a value. It will likewise be the only possible parent of the objects that populate it.

- **reference**

  a `reference` field accepts references to another object within the same root `Hit` instance. It cannot be set to an invalid ID, and likewise an object cannot be removed from the root instance if there are references to it. See (TODO: link) _mandatory validation_.

- **reference_array**

  likewise, a `reference_array` field accepts several references.

The following sub-chapters will explain how to use these value types. The examples will use the previously created `hit_test_file_model` instance.

## Setting a simple/scalar value

You can set a simple value using the (TODO : link) `Hit::set` method.

Example :

```rust
hit_instance::set(
  "my_id".into(),
  "name".into(),
  ObjectValue::String("my_instance_name".into())
).expect("This should set the root object name");
```

The action may return an error :

- (TODO: link): `HitError::IDNotFound` if the model id does not exist
- (TODO: link): `HitError::PropertyNotFound` if the model doesn't have the specified property
- (TODO: link): `HitError::InvalidDataType` if the model property doesn't accept that value

## Adding an object

You can add a subobject to an existing `object` using the `Hit::insert` method. You will need to provide :

- the `model_type` of the object, which is the id it is known as by the Kernel.
- the `String` id of the newly inserted object.
- You can provide values for some fields of the object in a `Hashmap<String, ObjectValue>`
- An object must always have a parent: you have to provide an id and a property name where to insert it
- You can insert an object in a specific position of the property it will be in, by specifiying the id of the object it will be isnerted before. If you don't provide an id (by using `None` in the function call), the object will be inserted at the end of the list.

Example :

```rust
hit_instance::insert(
  "file/folder".into(),
  "id2".into(),
  Hashmap::new(),
  IndexEntryProperty {
    id: "my_id".into(),
    property: "folders".into(),
  },
  None,
).expect("Insertion has failed");
```

## Removing an object

## Referencing an object

## Removing an object reference

# Guide : validation

`hit` provides validation for your data. There are two level of validation:

## Mandatory validation

There are some basic data integrity rules that `hit` models will not let you break. When you set a value or do an operation, if what you're doing violates these rules, the operation will return an error. `hit` is designed to have only a minimal amount of these errors. These errors are :

- Reference integrity. If you try to reference an object id that doesn't exist, or delete an object that is referenced in another field, your operation will not happen.
- Data types. If you're setting a field to an `ObjectValue` that is not accepted by its `FieldType`, your operation will be rejected.
- Object model names. If you're trying to insert an object with an invalid model name, your operation will be rejected.
- Object field names. If you're trying to set a property that doesn't exist on the model of an object, or to create an object with invalid property names.

**(TODO) : be able to add mandatory validation to a model.**

## Non-blocking validation

The main validation model is _non-blocking_ : that means you can assign invalid values to properties of your objects.

# Persisting `hit` data

## JSON import/export

## HitImporter/Exporter : create your own serializer/deserializer

# Plugins / Event handlers

TODO: write this chapter

# Guide: creating models

`hit` relies on Models. Similar to SQL table definitions, _Models_ are instances of the `Model` struct. A `Hit` instance relies on a `Kernel` which is a collection of models (and of plugins too as we'll see later). As a `Hit` instance has a hierarchical, tree-like structure, it must have a root object, which, like all of its sub-objects, is structured by a `Model`.

In our (TODO: link to github) `hit_test_file_model` example, the `file/filesystem` model is the root object, and contains files and folders sub-objects.

In that part of the guide we will introduce how to create your own kernel, models, as well as plugins.

## Model definitions

A `model` has the following properties:

- name
- definition
  This is a key/value dictionary. The definition is a struct that implements the `ModelField` trait. You can write your own Model Fields, but `hit` comes with standard ones, which match pretty obviously the types defined in the previous chapter :

  - (TODO: link) String
  - (TODO: link) Integer
  - (TODO: link) Float
  - (TODO: link) Date
  - (TODO: link) Subobject
  - (TODO: link) Subobject Array
  - (TODO: link) Reference
  - (TODO: link) Reference Array

## Model macro

## Validators

### Field validators

### Object validators

### Standard library

## Creating custom field types

## Kernel

### Kernel macro

TODO : create the macro ^^

# TODO : stabilization

- / write guide
- write rust doc
- test field structs ?
- validation for ref arrays
- test delete of object containing other objects

# TODO : After stabilization

- clarify API
- do we need insert quietly ?
- enums for string type
- unique in parent/parent.parent
  dependencies =>
- use linkedhashmap instead of hashmap to keep order (needed for numerotation plugin) ?

# TODO : less prioritary

- use rust enum for modifications
- refactor move function with allows_model
- implement ACID transactions ?
- integrate model_type index in Hit
- use strongly typed, yet extensible errors for validation ? How to do that ?

```

```
