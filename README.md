# hit

`hit` is a Rust library to handle data structured in tree-like documents with these features:

- **H**ierarchical
- **I**ndexed
- **T**yped

This library was intended to manage, in memory, deeply nested documents with strictly typed data structures and multiple inner links. That could be the representation of a word processor document, a directory and its files and subfolders with symbolic links...

## Hierarchical

Every document is structured like a document tree, in a similar way to MongoDB documents. That means a document always start with a root `object`, that is, a list of keys and values.

<!--
Here is an example

```json
{
  "_id": "my_id",
  "_type": "item/document",
  "hello": "world",
  "foo": 12,
  "items": [
    {
      "_id": "other_id",
      "_type": "item/name",
      "name": "other_name"
    },
    {
      "_id": "yet_other_id",
      "_type": "item/name",
      "name": "yet_other_name"
    }
  ]
}
``` -->

The values can be either be simple (string, numeric, date) values, or their can contain **other sub objects**. Every sub-object (except the root one) can thus be located as being in a **property** of another **object**.
(TODO : link to property types)

## Indexed

Every `object` (except, not yet implemented, embedded sub-objects) is indexed. That implies :

- every object must have an `_id` field, with a value of type `string`
- every object (except the root object) can be located using these three indices :
  - parent_id
  - parent_property
  - parent_position

The indexation allows `hit` to provide (TODO LINK) `reference` and `reference_array` type fields. They are inspired by foreign keys in relation databases, and enforce consistency rules : you cannot delete an `object` as long as there are references to it in the document.

The index also allows you to easily find all the references to an object.

## Typed

Every `object` in a document must have a (TODO: Link) `Model`. A model is identified by a string id, and is referenced in the `type` property of the `object`. To resolve model definitions from the ids, every instance of `hit` must be initialized with a `kernel` that contains the definitions.

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

To initiate a hit data instance, you need a (TODO: link) **kernel** with model definitions. One of the core kernels, officially supported by me, is the recursively designed (TODO: link) `hit_model`, which allows you to modelize models for `hit`.

To make it more simple, let's start with the basic, although completely useless (TODO: link) `hit_test_file_model`, that represents a directory/file structure, with links.

We will use ( TODO : link ) `Hit::new_with_values` to create the model. If you do not have initial values, you can instead use the (TODO : link ) `Hit::new` function.

```rust
use hit_file_model::create_kernel;
use hit::{ Hit, ObjectValue };
use std::collections::HashMap;

// create the kernel
let kernel = create_kernel();

// create a string id for the object
let id = "my_id".into();

// initiate the name value for our root object
let mut values = HashMap::new();
values.insert("name".into(), ObjectValue::String(name.to_string()));

// we can now create the hit instance
let my_model = Hit::new_with_values(
  id,
  kernel,
  values,
  // you must specify the main model name
  "file/filesystem"
);
```

## Property types

`hit` allows the following property types as values:

Simple values :

These values are set using the (TODO: link) `Hit::set` value.

- **string**

  a `string` field accepts rust `String` values.

- **number**

  a `number` field accepts rust `f32` values.

- **boolean**
- **date**

  a `data` field accepts timestamps as rust `i64` values.

Complex values :

These fields can only be populated using specific methods from the `Hit` struct.

- **sub_object**

  a `sub_object` field accepts a single `hit` object as a value. The field will be the only parent of the object that populates it.

- **sub_object_array**

  a `sub_object_array` field accepts several `hit` objects as a value. It will likewise be the only possible parent of the objects that populate it.

- **reference**

  a `reference` field accepts references to another object within the same root `Hit` instance. It cannot be set to an invalid ID, and likewise an object cannot be removed from the root instance if there are references to it. See (TODO: link) _mandatory validation_.

- **reference_array**

  likewise, a `reference_array` field accepts several references.

The following chapters will explain how to use these value types.

## Setting a simple value

## Adding an object

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

## Validators

### Field validators

### Object validators

### Standard library

## Creating custom field types

## Model macro

## Kernel

### Kernel macro

TODO : create the macro ^^

### Kernel Plugins

TODO: write this chapter

# TODO : stabilization

- / add serious tests
- / write guide
- write rust doc
- complete event/plugin system
- complete validation system

# TODO : After stabilization

- clarify API
- do we need insert quietly ?
- implement ACID transactions ?
- Should Kernel be typed/dynamic ? May be useful for extending though

- Allow interfaces in authorized models
- enums for string type
- unique in parent/parent.parent
  dependencies =>
