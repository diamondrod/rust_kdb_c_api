**!!Notice!!** This crate was migrated to [kdbplus](https://github.com/diamondrod/kdbplus) which has IPC module as well.

# Rust Wrapper of kdb+ C API

Programming language q (kdb+ is a database written in q) is providing only C API but sometimes an external library provides Rust interface but not C/C++ interface. From the fame of its performance, Rust still should be feasible to build a shared library for kdb+. This library is provided to address such a natural demand (desire, if you will). Since there is no way for everyone but creating a wrapper like this to write a shared library for kdb+, it probably make sense for someone to provide the wrapper, and it was done here.

In order to avoid writing too large `unsafe` block which leads to poor optimization, most of native C API functions are provided with a wrapper funtion with a bit of ergonomic safety and with intuitive implementation as a trait method. The only exceptions are `knk` and `k` which are using elipsis (`...`) as its argument. These functions are provided under `native` namespace with the other C API functions.

## Installation

Use `kdb_c_api` as a library name in `Cargo.toml`.

```toml

[dependencies]
kdb_c_api="^0.1"

```

## Examples

The examples of using C API wrapper are included in `c_api_examples` folder. The examples are mirroring the examples in the document of `kdb_c_api` library and the functions are also used for simple tests of the library. The test is conducted in the `test.q` under `tests/` by loading the functions defined in a shared library built from the examples.

Here are some examples:

### C API Style

```rust
use kdb_c_api::*;
use kdb_c_api::native::*;

#[no_mangle]
pub extern "C" fn create_symbol_list(_: K) -> K{
  unsafe{
    let mut list=ktn(qtype::SYMBOL as i32, 0);
    js(&mut list, ss(str_to_S!("Abraham")));
    js(&mut list, ss(str_to_S!("Isaac")));
    js(&mut list, ss(str_to_S!("Jacob")));
    js(&mut list, sn(str_to_S!("Josephine"), 6));
    list
  }
}
 
#[no_mangle]
pub extern "C" fn catchy(func: K, args: K) -> K{
  unsafe{
    let result=ee(dot(func, args));
    if (*result).qtype == qtype::ERROR{
      println!("error: {}", S_to_str((*result).value.symbol));
      // Decrement reference count of the error object
      r0(result);
      KNULL
    }
    else{
      result
    }
  }
}

#[no_mangle]
pub extern "C" fn dictionary_list_to_table() -> K{
  unsafe{
    let dicts=knk(3);
    let dicts_slice=dicts.as_mut_slice::<K>();
    for i in 0..3{
      let keys=ktn(qtype::SYMBOL as i32, 2);
      let keys_slice=keys.as_mut_slice::<S>();
      keys_slice[0]=ss(str_to_S!("a"));
      keys_slice[1]=ss(str_to_S!("b"));
      let values=ktn(qtype::INT as i32, 2);
      values.as_mut_slice::<I>()[0..2].copy_from_slice(&[i*10, i*100]);
      dicts_slice[i as usize]=xD(keys, values);
    }
    // Format list of dictionary as a table.
    // ([] a: 0 10 20i; b: 0 100 200i)
    k(0, str_to_S!("{[dicts] -1 _ dicts, (::)}"), dicts, KNULL)
  } 
}
```

q can use these functions like this:

```q
q)summon:`libc_api_examples 2: (`create_symbol_list; 1)
q)summon[]
`Abraham`Isaac`Jacob`Joseph
q)`Abraham`Isaac`Jacob`Joseph ~ summon[]
q)catchy: `libc_api_examples 2: (`catchy; 2);
q)catchy[$; ("J"; "42")]
42
q)catchy[+; (1; `a)]
error: type
q)unfortunate_fact: `libc_api_examples 2: (`dictionary_list_to_table; 1);
q)unfortunate_fact[]
a  b  
------
0  0  
10 100
20 200
```

### Rust Style

The examples below are written without `unsafe` code. You can see how comfortably breathing are the wrapped functions in the code.

```rust
use kdb_c_api::*;

#[no_mangle]
pub extern "C" fn create_symbol_list2(_: K) -> K{
  let mut list=new_simple_list(qtype::SYMBOL, 0);
  list.push_symbol("Abraham").unwrap();
  list.push_symbol("Isaac").unwrap();
  list.push_symbol("Jacob").unwrap();
  list.push_symbol_n("Josephine", 6).unwrap();
  list
}

#[no_mangle]
fn no_panick(func: K, args: K) -> K{
  let result=error_to_string(apply(func, args));
  if result.get_type() == qtype::ERROR{
    println!("FYI: {}", result.get_symbol().unwrap());
    // Decrement reference count of the error object which is no longer used.
    decrement_reference_count(result);
    KNULL
  }
  else{
    println!("success!");
    result
  }
}

#[no_mangle]
pub extern "C" fn create_table2(_: K) -> K{
  // Build keys
  let keys=new_simple_list(qtype::SYMBOL, 2);
  let keys_slice=keys.as_mut_slice::<S>();
  keys_slice[0]=internalize(str_to_S!("time"));
  keys_slice[1]=internalize_n(str_to_S!("temperature_and_humidity"), 11);

  // Build values
  let values=new_simple_list(qtype::COMPOUND, 2);
  let time=new_simple_list(qtype::TIMESTAMP, 3);
  // 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392
  time.as_mut_slice::<J>().copy_from_slice(&[119067859167018272_i64, 201766609419710368, 271897944018691392]);
  let temperature=new_simple_list(qtype::FLOAT, 3);
  temperature.as_mut_slice::<F>().copy_from_slice(&[22.1_f64, 24.7, 30.5]);
  values.as_mut_slice::<K>().copy_from_slice(&[time, temperature]);
  
  flip(new_dictionary(keys, values))
}
```

And q code is here:

```q
q)summon:`libc_api_examples 2: (`create_symbol_list2; 1)
q)summon[]
`Abraham`Isaac`Jacob`Joseph
q)chill: `libc_api_examples 2: (`no_panick; 2);
q)chill[$; ("J"; "42")]
success!
42
q)chill[+; (1; `a)]
FYI: type
q)climate_change: libc_api_examples 2: (`create_table2; 1);
q)climate_change[]
time                          temperature
-----------------------------------------
2003.10.10D02:24:19.167018272 22.1       
2006.05.24D06:16:49.419710368 24.7       
2008.08.12D23:12:24.018691392 30.5  
```

## Test

Tests are conducted with the example functions in `tests/test.q` by loading the functions into q process.

```bash

rust_kdb_c_api]$ cargo build
rust_kdb_c_api]$ cp target/debug.libc_api_examples.so tests/
rust_kdb_c_api]$ cd tests
tests]$ q test.q
Initialized something, probably it is your mindset.
bool: true
bool: false
byte: 0xc4
GUID: 8c6b-8b-64-68-156084
short: 10
int: 42
int: 122
int: 7336
int: 723
int: 14240
int: 2056636
long: -109210
long: 43200123456789
long: -325389000000021
real: 193810.31
float: -37017.09330000
float: 742.41927468
char: "k"
symbol: `locust
string: "gnat"
string: "grasshopper"
error: type
What do you see, son of man?: a basket of summer fruit
What do you see, son of man?: boiling pot, facing away from the north
symbol: `rust
success!
FYI: type
おいしい！
おいしい！
おいしい！
おいしい！
おいしい！
おいしい！
おいしい！
おいしい！
おいしい！
おいしい！
"Collect the clutter of apples!"
test result: ok. 122 passed; 0 failed
What are the three largest elements?: `belief`love`hope
q)

```

## Document

The document of this crate itself is on the [crates.io page](https://crates.io.docs/kdb_c_api).

For details of C API itself, check the documents of KX website.

- [Refernce](https://code.kx.com/q/interfaces/capiref/)
- [Memory management](https://code.kx.com/q/interfaces/c-client-for-q/#managing-memory-and-reference-counting)

## Note

This library is purposed to be used to build a sared library; therefore some unrelated functions are removed. For example, connection functions to kdb+ like `khpu` are not included.
