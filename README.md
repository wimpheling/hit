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

The index also allows you to easily find all the references to an object. (TOO : does it ?)

## Typed

Every `object` in a document must have a (TODO: Link) `Model`. A model is identified by a string id, and is referenced in the `type` property of the `object`. To resolve model definitions from the ids, every instance of `hit` must be initialized with a `kernel` that contains the definitions.

The models :

- list the names of the accepted fields of an object
- restrict the accepted values using `field types` (TODO: link) and - optionally - `validators` (TODO : link)

# Get started

`hit` is

<!-- > We will sometimes use JSON representations of the `hit` documents. This is chosen for readability as it is a concise and well-known format, but JSON is not the native format of `hit`. A JSON serializer/deserializer is available, but the output we show here is not exactly the same as the JSON serializer output.
 -->

# Guide : How to create and use a `hit` instance

## Creating a `hit` instance

To initiate a hit data instance, you need a **kernel** with model definitions. One of the core kernels, officially supported by me, is the recursively designed `hit_model`, which allows you to modelize models for `hit`.

![I heard you liked models](./img/xzibit.jpg)

To make it more simple, let's start with the basic, although completely useless `file_model`, that represents a directory/file structure, with links.

We will use ( TODO : link ) `Hit::new_with_values` to create the model. If you do not have initial values, you can instead use the (TODO : link ) `Hit::new` function.

```rust
use file_model::create_kernel;
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
  "file_model/project"
);
```

## Property types

`hit` allows the following property types as values:

Simple values :

- **string**
- **number**
- **boolean**
- **date**

Complex values :

- **sub_object**
- **sub_object_array**
- **reference**
- **reference_array**

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

# Models

## Model definitions

A `model` has the following properties:

- name
- definition
  This is a key/value dictionary. The definition is a struct that implements the `ModelField` trait. You can write your own Model Fields, but `hit` comes with standard ones:

  - String
  - Integer
  - TODO

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

# TODO

- / add serious tests
- / write guide
- write rust doc
- clarify API
- complete event/plugin system
- Have consistent errors
  https://nick.groenen.me/posts/rust-error-handling/
- publish to crates.io

# TODO After stabilization

- do we need insert quietly ?
- implement ACID
- Should Kernel be typed/dynamic ? May be useful for extending though
