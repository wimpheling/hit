# indexed_model

This is a Rust library to handle data structured in tree-like documents with these features:

This library was intended to manage, in memory, deeply nested documents with strictly typed data structures and multiple inner links. That could be the representation of a word processor document, a directory and its files and subfolers with symbolic links...

# Get started

# Philosophy

`indexed_model` is

- hierarchical
- indexed
- relational
- typed

<!-- > We will sometimes use JSON representations of the `indexed_model` documents. This is chosen for readability as it is a concise and well-known format, but JSON is not the native format of `indexed_model`. A JSON serializer/deserializer is available, but the output we show here is not exactly the same as the JSON serializer output.
 -->

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

That means every sub-object exists and has a position in a parent collection, that can be defined by it's subobject ID, it's property name, and its index in that collection.

(TODO : link to property types)

## Indexed

Every `object` (except, not yet implemented, embedded sub-objects) is indexed. That implies :

- every object must have an `_id` field, with a value of type `string`
- every object can be located using these three indices :
  - parent_id
  - parent_property
  - parent_position

## Relational

The indexation allows `indexed_model` to provide `reference` and `reference_array` type fields. They are inspired by foreign keys in relation databases, in the sense that :

- you cannot delete an `object` as long as there are references to it in the document.

## Typed

Every `object` in a document must have a `model`. A model is identified by a string id, and is referenced in the `type` property of the `object`. To resolve model definitions from the ids, every instance of `indexed_model` must be initialized with a `kernel` that contains the definitions.

The models :

- list the names of the accepted fields of an object
- restrict the accepted values using `field types` (TODO: link) and - optionally - `validators` (TODO : link)

# Documentation

## Property types

`indexed_model` allows the following property types as values:

- **string**
- **number**
- **boolean**
- **date**
- **sub_object**
- **sub_object_array**
- **reference**
- **reference_array**

## Model definitions

A `model` has the following properties:

- name
- definition
  This is a key/value dictionary. The definition is a struct that implements the `ModelField` trait. You can write your own Model Fields, but `indexed_model` comes with standard ones:

  - String
  - Integer
  - TODO

# TODO

- / add serious tests
- / write guide
- write doc
- clarify API
- complete event/plugin system
- Have consistent errors
  https://nick.groenen.me/posts/rust-error-handling/
- publish to crates.io
- implement ACID
