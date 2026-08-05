# Markdown Docs for Eartha's Frontier

Hello, this file is the main documentation for how things work in Eartha's Frontier.

## Main.rs - Program's main event loop

## Config.rs - Starting configuration logic

## Loader.rs - Loading saved data from storage

## Auth.rs - Everything authentication and authorization

## Elements.rs - Defining core data structures and traits

The elements file is a massive one, but is split by intra-file modules that imply purpose.

### Core module

The core module defines the traits that the entire program revolves around. As of right now, two traits are defined.

First is the EFComponent trait. Any struct that implements the EFComponent trait is a struct that can safely be used throughout the program. This trait requires that an object of a struct type must print a string saying what type it is. For example, a database component would return a String of "database."

Second is the EFByteRepCompatible trait. Any struct that implements the EFByteRepCompatible trait is a struct where the object can be transformed to and from a vector of bytes. It correlates with the struct EFByteRep, which itself is a object that contains a vector of u8 and a string that holds the component type.

#### Things to consider when implementing EFByteRepCompatible

As EFByteRep is nothing more than a vector of bytes, how you decide to encode the information is completely up to you. A vector is used for flexibility purposes, but in the future, a more "professional" type may be used.

How I personally implement EFByteRepCompatible takes inspiration from alot of network encodings out there:

##### For encoding to EFByteRep

1. Use the first set of bytes to store offsets. For a struct with three attributes, Use the first three u8 indexes to store the offset of each where each attribute's byte data begins in the vector.
1. Convert each attribute into its own vector of bytes (which is probably wasteful). If you have an attribute that is an Option, what you do is you check if's the attribute is set to Some or None. If it's set to Some, turn the value inside into a vector of u8. If it's set to None, create empty vector with Vec::new() and set the offset number to something that can signal such. I set the offset to 0, because index 0 is guarenteed to be used for an offset in my encoding strategy.
1. To calculate the offsets, set the first offset to the first index used for data. To get the second offset, add the length of the byte vector used for the first attribute stored to the first offset. Repeat this process until you have your offsets.
1. Lastly, create your final vector that you are going to return. I like to start with a vector macro of the offsets, for example vec![offset_1, offset_2, offset_3]. Then I use the append function on the vector to keep adding the bytes of the attributes in the order I want.
1. Return an EFByteRep object with your final vector and the name of your component type.

If one or more of your attributes are struct types themselves, make the struct that they are EFByteRepCompatible, and just use their to_byte_rep function to get their vector of bytes to add.

##### For decoding to your struct

Once you have your encoding, the decoding process kinda writes itself, although I do some cheeky stuff.

1. First, check the component attribute to make sure you're working with expected data. If not, return an error.
1. Next, use a cheeky tuple to unpack the first set of bytes as done below:
```
let (one_start, two_start, three_start) = match br.bytes.get(0..3) {
    Some(&[one, two, three]) => (one as usize, two as usize, three as usize),
    ...
}
```
1. For each attribute, use a match pattern that retrieves the corresponding set of bytes using br.bytes.get(one_start..two_start), where:
    1. one_start corresponds to where the bytes for the first attribute start.
    1. two_start corresponds to where the bytes for the second attribute start.
1. two_start is exclusive, so you only get the byte slice for the first attribute. Work from there to create your attribute from that.
1. Repeat this process for the rest of the attributes, using br.bytes.get(last_start..) to get the rest of the data for the last attribute, or not if this happened to be an Option and the object that got turned into bytes set this attribute to None.

### Primitives module

### Common module

### Entity module

### Components module

## Utils.rs - General-purpose functions

## How Command Line Arguments work

You can specify arguments one of two ways

1. "-key value" where the key is prefixed with a hyphen and a value is typed after with a space in between.
1. "key=value" where the key and value are one string, connected with a equal sign.

You can mix and match both values, but these are the only ways to do it.

## Starting Configuration Priority

1. Command line arguments have top priority
1. If a startup configuration argument file is provided, the file's arguments take second priority.

## How the identification works
The ID in Eartha's Frontier is a combination of three CPU-architecture compatible numbers (32-bit or 64-bit). 

1. System Level - The level at which the entity sits in the system. The higher the level, the farther it is from the root of the system. A system level of zero means that the entity is at the very root of the system.
1. Parent ID - The ID of the parent entity that owns the entity. It is gotten from an autoincrementing number, with the ID of zero reserved for the parent of the entity.
1. Local ID - The auto-incrementing number 