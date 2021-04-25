//! This module is provided as examples of `kdb_c_api` crate. The functions defined here will be
//!  used for simple tests.

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                            Load Libraries                            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

extern crate kdb_c_api;

use kdb_c_api::*;
use kdb_c_api::native::*;
use std::ffi::c_void;
use libc::{pipe, send};

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          Global Variables                            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// Example of `KUNLL`.
#[no_mangle]
pub extern "C" fn vanity(_: K) -> K{
  println!("Initialized something, probably it is your mindset.");
  KNULL
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                              Macros                                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Utlity %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Example of `str_to_S`.
#[no_mangle]
pub extern "C" fn pingpong(_: K) -> K{
  unsafe{
    native::k(0, str_to_S!("ping"), new_int(77), KNULL)
  }
}

/// Example of `null_terminated_str_to_const_S`.
#[no_mangle]
pub extern "C" fn must_be_int(obj: K) -> K{
  unsafe{
    if (*obj).qtype != -qtype::INT{
      krr(null_terminated_str_to_const_S("not an int\0"))
    }
    else{
      KNULL
    }
  }
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                             K Utility                                //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// Example of `as_mut_slice`.
#[no_mangle]
pub extern "C" fn modify_long_list_a_bit(long_list: K) -> K{
  if long_list.len() >= 2{
    // Derefer as a mutable i64 slice.
    long_list.as_mut_slice::<J>()[1]=30000_i64;
    // Increment the counter for reuse on q side.
    increment_reference_count(long_list)
  }
  else{
    new_error("this list is not long enough. how ironic...\0")
  } 
}

#[no_mangle]
pub extern "C" fn print_bool(atom: K) -> K{
  match atom.get_bool(){
    Ok(boolean) => {
      println!("bool: {}", boolean);
      KNULL
    },
    Err(error) => new_error(error)
  }
}

#[no_mangle]
pub extern "C" fn print_guid(atom: K) -> K{
  match atom.get_guid(){
    Ok(guid) => {
      let strguid=guid.iter().map(|b| format!("{:02x}", b)).collect::<String>();
      println!("GUID: {}-{}-{}-{}-{}", &strguid[0..4], &strguid[4..6], &strguid[6..8], &strguid[8..10], &strguid[10..16]);
      KNULL
    },
    Err(error) => new_error(error)
  }
}

/// Example of `get_byte`.
#[no_mangle]
pub extern "C" fn print_byte(atom: K) -> K{
  match atom.get_byte(){
    Ok(byte) => {
      println!("byte: {:#4x}", byte);
      KNULL
    },
    Err(error) => new_error(error)
  }
}

/// Example of `get_short`.
#[no_mangle]
pub extern "C" fn print_short(atom: K) -> K{
  match atom.get_short(){
    Ok(short) => {
      println!("short: {}", short);
      KNULL
    },
    Err(error) => new_error(error)
  }
}

/// Example of `get_int`.
#[no_mangle]
pub extern "C" fn print_int(atom: K) -> K{
  match atom.get_int(){
    Ok(int) => {
      println!("int: {}", int);
      KNULL
    },
    Err(error) => new_error(error)
  }
}

/// Example of `get_long`.
#[no_mangle]
pub extern "C" fn print_long(atom: K) -> K{
  match atom.get_long(){
    Ok(long) => {
      println!("long: {}", long);
      KNULL
    },
    Err(error) => new_error(error)
  }
  
}

/// Example of `get_real`.
#[no_mangle]
pub extern "C" fn print_real(atom: K) -> K{
  match atom.get_real(){
    Ok(real) => {
      println!("real: {}", real);
      KNULL
    },
    Err(error) => new_error(error)
  }
}

/// Example of `get_float`.
#[no_mangle]
pub extern "C" fn print_float(atom: K) -> K{
  match atom.get_float(){
    Ok(float) => {
      println!("float: {:.8}", float);
      KNULL
    },
    Err(error) => new_error(error)
  }
}

/// Example of `get_char`.
#[no_mangle]
pub extern "C" fn print_char(atom: K) -> K{
  match atom.get_char(){
    Ok(character) => {
      println!("char: \"{}\"", character);
      KNULL
    },
    Err(error) => new_error(error)
  }
}

/// Example of `get_symbol`.
#[no_mangle]
pub extern "C" fn print_symbol2(atom: K) -> K{
  match atom.get_symbol(){
    Ok(symbol) => {
      println!("symbol: `{}", symbol);
      KNULL
    },
    Err(error) => new_error(error)
  }
}

/// Example of `get_str`.
#[no_mangle]
pub extern "C" fn print_string(string: K) -> K{
  match string.get_str(){
    Ok(string_) => {
      println!("string: \"{}\"", string_);
      KNULL
    },
    Err(error) => new_error(error)
  }
}

/// Example of `get_string`.
#[no_mangle]
pub extern "C" fn print_string2(string: K) -> K{
  match string.get_string(){
    Ok(string_) => {
      println!("string: \"{}\"", string_);
      KNULL
    },
    Err(error) => new_error(error)
  }
}

/// Example of `append`.
#[no_mangle]
pub extern "C" fn concat_list2(mut list1: K, list2: K) -> K{
  if let Err(err) = list1.append(list2){
    new_error(err)
  }
  else{
    increment_reference_count(list1)
  }
}

/// Example of `push`.
#[no_mangle]
pub extern "C" fn create_compound_list2(int: K) -> K{
  let mut list=new_simple_list(qtype::COMPOUND, 0);
  for i in 0..5{
    list.push(new_long(i)).unwrap();
  }
  list.push(increment_reference_count(int)).unwrap();
  list
}

#[no_mangle]
pub extern "C" fn create_simple_list2(_: K) -> K{
  let mut list=new_simple_list(qtype::DATE, 0);
  for i in 0..5{
    list.push_raw(i).unwrap();
  }
  list
}

/// Example of `push_symbol`.
#[no_mangle]
pub extern "C" fn create_symbol_list2(_: K) -> K{
  let mut list=new_simple_list(qtype::SYMBOL, 0);
  list.push_symbol("Abraham").unwrap();
  list.push_symbol("Isaac").unwrap();
  list.push_symbol("Jacob").unwrap();
  list.push_symbol_n("Josephine", 6).unwrap();
  list
}

/// Example of `q_ipc_decode`.
#[no_mangle]
pub extern "C" fn encrypt(object: K)->K{
  match object.q_ipc_encode(3){
    Ok(bytes) => bytes,
    Err(error) => new_error(error)
  }
}

/// Example of `q_ipc_encode`.
#[no_mangle]
pub extern "C" fn decrypt(bytes: K)->K{
  match bytes.q_ipc_decode(){
    Ok(object) => object,
    Err(error) => new_error(error)
  }
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                            Constructors                              //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// Example of `kb`.
#[no_mangle]
pub extern "C" fn create_bool(_: K) -> K{
  unsafe{kb(1)}
}

/// Example of `kb`.
#[no_mangle]
pub extern "C" fn create_bool2(_: K) -> K{
  new_bool(1)
}

/// Example of `ku`.
#[no_mangle]
pub extern "C" fn create_guid(_: K) -> K{
  unsafe{ku(U::new([0x1e_u8, 0x11, 0x17, 0x0c, 0x42, 0x24, 0x25, 0x2c, 0x1c, 0x14, 0x1e, 0x22, 0x4d, 0x3d, 0x46, 0x24]))}
}

/// Example of `kg`.
#[no_mangle]
pub extern "C" fn create_byte(_: K) -> K{
  unsafe{kg(0x3c)}
}

/// Example of `kh`.
#[no_mangle]
pub extern "C" fn create_short(_: K) -> K{
  unsafe{kh(-144)}
}

/// Example of `ki`.
#[no_mangle]
pub extern "C" fn create_int(_: K) -> K{
  unsafe{ki(86400000)}
}

/// Example of `kj`.
#[no_mangle]
pub extern "C" fn create_long(_: K) -> K{
  unsafe{kj(-668541276001729000)}
}

/// Example of `ke`.
#[no_mangle]
pub extern "C" fn create_real(_: K) -> K{
  unsafe{ke(0.00324)}
}

/// Example of `kf`.
#[no_mangle]
pub extern "C" fn create_float(_: K) -> K{
  unsafe{kf(-6302.620)}
}

/// Example of `kc`.
#[no_mangle]
pub extern "C" fn create_char(_: K) -> K{
  unsafe{kc('q' as I)}
}

/// Example of `ks`.
#[no_mangle]
pub extern "C" fn create_symbol(_: K) -> K{
  unsafe{ks(str_to_S!("symbolism"))}
}

/// Example of `ktj` for timestamp.
#[no_mangle]
pub extern "C" fn create_timestamp(_: K) -> K{
  // 2015.03.16D00:00:00:00.000000000
  unsafe{ktj(-qtype::TIMESTAMP as I, 479779200000000000)}
}

/// Example of `ktj` for timespan.
#[no_mangle]
pub extern "C" fn create_timespan(_: K) -> K{
  // -1D01:30:00.001234567
  unsafe{ktj(-qtype::TIMESPAN as I, -91800001234567)}
}

/// Example of `kd`.
#[no_mangle]
pub extern "C" fn create_date(_: K) -> K{
  // 1999.12.25
  unsafe{kd(-7)}
}

/// Example of `kz`.
#[no_mangle]
pub extern "C" fn create_datetime(_: K) -> K{
  // 2015.03.16T12:00:00:00.000
  unsafe{kz(5553.5)}
}

/// Example of `kt`.
#[no_mangle]
pub extern "C" fn create_time(_: K) -> K{
  // -01:30:00.123
  unsafe{kt(-5400123)}
}

/// Example of `kp`.
#[no_mangle]
pub extern "C" fn create_string(_: K) -> K{
  unsafe{kp(str_to_S!("this is a text."))}
}

/// Example of `kpn`.
#[no_mangle]
pub extern "C" fn create_string2(_: K) -> K{
  unsafe{kpn(str_to_S!("The meeting was too long and I felt it s..."), 24)}
}

/// Example of `xT`.
#[no_mangle]
pub extern "C" fn create_table(_: K) -> K{
  unsafe{
    let keys=ktn(qtype::SYMBOL as I, 2);
    let keys_slice=keys.as_mut_slice::<S>();
    keys_slice[0]=ss(str_to_S!("time"));
    keys_slice[1]=ss(str_to_S!("temperature"));
    let values=knk(2);
    let time=ktn(qtype::TIMESTAMP as I, 3);
    // 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392
    time.as_mut_slice::<J>().copy_from_slice(&[119067859167018272_i64, 201766609419710368, 271897944018691392]);
    let temperature=ktn(qtype::FLOAT as I, 3);
    temperature.as_mut_slice::<F>().copy_from_slice(&[22.1_f64, 24.7, 30.5]);
    values.as_mut_slice::<K>().copy_from_slice(&[time, temperature]);
    xT(xD(keys, values))
  }
}

/// Example of `ktd`.
#[no_mangle]
pub extern "C" fn keyed_to_simple_table(dummy: K) -> K{
  unsafe{ktd(create_keyed_table(dummy))}
}

/// Example of `xD`.
#[no_mangle]
pub extern "C" fn create_dictionary(_: K) -> K{
  let keys=unsafe{ktn(qtype::INT as I, 2)};
  keys.as_mut_slice::<I>().copy_from_slice(&[0, 1]);
  let values=unsafe{knk(2)};
  let date_list=unsafe{ktn(qtype::DATE as I, 3)};
  // 2000.01.01 2000.01.02 2000.01.03
  date_list.as_mut_slice::<I>()[0..3].copy_from_slice(&[0, 1, 2]);
  let string=unsafe{kp(str_to_S!("I'm afraid I would crash the application..."))};
  values.as_mut_slice::<K>()[0..2].copy_from_slice(&[date_list, string]);
  // 0 1i!(2000.01.01 2000.01.02 2000.01.03; "I'm afraid I would crash the application...")
  unsafe{xD(keys, values)}
}

/// Example of `knt`.
#[no_mangle]
pub extern "C" fn create_keyed_table(dummy: K) -> K{
  unsafe{knt(1, create_table(dummy))}
}

/// Example of `krr`.
#[no_mangle]
pub extern "C" fn thai_kick(_: K) -> K{
  unsafe{
    krr(null_terminated_str_to_const_S("Thai kick unconditionally!!\0"))
  }
}

/// Example of `ja`.
#[no_mangle]
pub extern "C" fn create_simple_list(_: K) -> K{
  let mut list=unsafe{ktn(qtype::TIMESTAMP as I, 0)};
  for i in 0..5{
    let mut timestamp=86400000000000 * i as J;
    unsafe{ja(&mut list, std::mem::transmute::<*mut J, *mut V>(&mut timestamp));}
  }
  list
}

/// Example of `jv`.
#[no_mangle]
pub extern "C" fn concat_list(mut list1: K, list2: K) -> K{
  unsafe{
    jv(&mut list1, list2);
    r1(list1)
  }
}

/// Example of `js`.
#[no_mangle]
pub extern "C" fn create_symbol_list(_: K) -> K{
  unsafe{
    let mut list=ktn(qtype::SYMBOL as I, 0);
    js(&mut list, ss(str_to_S!("Abraham")));
    js(&mut list, ss(str_to_S!("Isaac")));
    js(&mut list, ss(str_to_S!("Jacob")));
    js(&mut list, sn(str_to_S!("Josephine"), 6));
    list
  }
}

/// Example of `jk`.
#[no_mangle]
pub extern "C" fn create_compound_list(_: K) -> K{
  unsafe{
    let mut list=knk(0);
    jk(&mut list, ks(str_to_S!("1st")));
    jk(&mut list, ki(2));
    jk(&mut list, kpn(str_to_S!("3rd"), "3rd".chars().count() as i64));
    list
  }
}

/// Example of `ee`. 
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

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                            IPC Functions                             //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// Example of `k`.
#[no_mangle]
pub extern "C" fn dictionary_list_to_table() -> K{
  let dicts=unsafe{knk(3)};
  let dicts_slice=dicts.as_mut_slice::<K>();
  for i in 0..3{
    let keys=unsafe{ktn(qtype::SYMBOL as i32, 2)};
    let keys_slice=keys.as_mut_slice::<S>();
    keys_slice[0]=unsafe{ss(str_to_S!("a"))};
    keys_slice[1]=unsafe{ss(str_to_S!("b"))};
    let values=unsafe{ktn(qtype::INT as i32, 2)};
    values.as_mut_slice::<I>()[0..2].copy_from_slice(&[i*10, i*100]);
    dicts_slice[i as usize]=unsafe{xD(keys, values)};
  }
  // Format list of dictionary as a table. 
  // ([] a: 0 10 20i; b: 0 100 200i)
  unsafe{k(0, str_to_S!("{[dicts] -1 _ dicts, (::)}"), dicts, KNULL)} 
}

/// Example of `b9`.
#[no_mangle]
pub extern "C" fn conceal(object: K)->K{
  unsafe{b9(3, object)}
}

/// Example of `d9`.
#[no_mangle]
pub extern "C" fn reveal(bytes: K)->K{
  unsafe{d9(bytes)}
}

/// Callback function to send asynchronous query to a q process which sent a query to the
///  caller of this function.
extern "C" fn counter(socket: I) -> K{
  let extra_query="show `$\"Counter_punch!!\"".as_bytes();
  let query_length=extra_query.len();
  // header (8) + list header (6) + data length
  let total_length=8+6+query_length;
  // Buffer
  let mut message: Vec<u8>=Vec::with_capacity(total_length);
  // Little endian, async, uncompress, reserved
  message.extend_from_slice(&[1_u8, 0, 0, 0]);
  // Total message length
  message.extend_from_slice(&(total_length as i32).to_le_bytes());
  // Data type, attribute
  message.extend_from_slice(&[10_u8, 0]);
  // Length of data
  message.extend_from_slice(&(query_length as i32).to_le_bytes());
  // Data
  message.extend_from_slice(extra_query);
  // Send
  unsafe{send(socket, message.as_slice().as_ptr() as *const c_void, total_length, 0)};
  KNULL
}

/// Example of `sd1`.
#[no_mangle]
pub extern "C" fn enable_counter(socket: K) -> K{
  unsafe{
    let result=sd1(socket.get_int().expect("oh no"), counter);
    if result.get_type()== qtype::NULL || result.get_type()== qtype::ERROR{
      return krr(null_terminated_str_to_const_S("Failed to hook\0"));
    }
    else{
      KNULL
    }
  }
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                           Reference Count                            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// Example of `r0`.
#[no_mangle]
pub extern "C" fn idle_man(_: K) -> K{
  unsafe{
    // Creare an int object.
    let int=ki(777);
    // Changed the mind. Discard it.
    r0(int);
  }
  // Return null.
  KNULL
}

/// Example of `r1`.
#[no_mangle]
pub extern "C" fn pass_through_cave(pedestrian: K) -> K{
  unsafe{
    let item=k(0, str_to_S!("get_item1"), r1(pedestrian), KNULL);
    println!("What do you see, son of man?: {}", item.get_str().expect("oh no"));
    r0(item);
    let item=k(0, str_to_S!("get_item2"), r1(pedestrian), KNULL);
    println!("What do you see, son of man?: {}", item.get_str().expect("oh no"));
    r0(item);
    r1(pedestrian)
  }
}
  
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                            Miscellaneous                             //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// Example of `dot`.
#[no_mangle]
pub extern "C" fn rust_parse(dollar: K, type_and_text: K) -> K{
  unsafe{
    dot(dollar, type_and_text)
  }
}

/// Example of `setm`.
#[no_mangle]
pub extern "C" fn parallel_sym_change(list: K) -> K{
  unsafe{
    // `K` cannot have `Send` because it is a pointer but `k0` does.
    let mut inner=*list;
    // Lock symbol before creating an internal symbol on another thread.
    setm(1);
    let task=std::thread::spawn(move || {
      inner.as_mut_slice::<S>()[0]=ss(str_to_S!("replaced"));
      inner
    });
    list.as_mut_slice::<S>()[1]=ss(str_to_S!("symbolbol"));
    match task.join(){
      Err(_) => {
        // Unlock.
        setm(0);
        krr(null_terminated_str_to_const_S("oh no\0"))
      },
      Ok(l) => {
        // Unlock.
        setm(0);
        (*list)=l;
        // Increment reference count for copy.
        r1(list)
      }
    }
  }
}

/// Example of `ymd`.
#[no_mangle]
pub extern "C" fn ymd_to_days(_: K) -> K{
  unsafe{
    let days=ymd(2020, 4, 1);
    ki(days) 
  }
}

/// Example of `dj`.
#[no_mangle]
pub extern "C" fn days_to_date(days: K) -> K{
  unsafe{
    let number=dj(days.get_int().expect("oh no"));
    ki(number)
  }
}

/// Example of `S_to_str`.
#[no_mangle]
pub extern "C" fn print_symbol(symbol: K) -> K{
  unsafe{
    if (*symbol).qtype == -qtype::SYMBOL{
      println!("symbol: `{}", S_to_str((*symbol).value.symbol));
    }
    // return null
    KNULL
  }
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                              Utility                                 //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// Example of `null_terminated_str_to_S`.
#[no_mangle]
pub extern "C" fn pingpong2(_: K) -> K{
  unsafe{
    native::k(0, null_terminated_str_to_S("ping\0"), new_int(77), KNULL)
  }
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                              Re-export                               //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

#[no_mangle]
pub extern "C" fn create_char2(_: K) -> K{
  new_char('t')
}

#[no_mangle]
pub extern "C" fn create_symbol2(_: K) -> K{
  new_symbol("symbolic")
}

#[no_mangle]
pub extern "C" fn create_timestamp2(_: K) -> K{
  // 2015.03.16D00:00:00:00.000000000
  new_timestamp(479779200000000000)
}

/// Example of `new_month`.
#[no_mangle]
pub extern "C" fn create_month(_: K) -> K{
  // 2010.07m
  new_month(126)
}

#[no_mangle]
pub extern "C" fn create_timespan2(_: K) -> K{
  // -1D01:30:00.001234567
  new_timespan(-91800001234567)
}

/// Example of `new_minute`.
#[no_mangle]
pub extern "C" fn create_minute(_: K) -> K{
  // 10:40
  new_minute(640)
}

/// Example of `new_second`.
#[no_mangle]
pub extern "C" fn create_second(_: K) -> K{
  // -02:00:00
  new_second(-7200)
}

/// Example of `new_error`.
#[no_mangle]
pub extern "C" fn keep_out(_: K) -> K{
  new_error("No means no.\0")
}

/// Example of `error_to_string`.
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

//%% Table %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

#[no_mangle]
pub extern "C" fn create_table2(_: K) -> K{
  let keys=new_simple_list(qtype::SYMBOL, 2);
  let keys_slice=keys.as_mut_slice::<S>();
  keys_slice[0]=internalize(str_to_S!("time"));
  keys_slice[1]=internalize_n(str_to_S!("temperature_and_humidity"), 11);
  let values=new_simple_list(qtype::COMPOUND, 2);
  let time=new_simple_list(qtype::TIMESTAMP, 3);
  // 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392
  time.as_mut_slice::<J>().copy_from_slice(&[119067859167018272_i64, 201766609419710368, 271897944018691392]);
  let temperature=new_simple_list(qtype::FLOAT, 3);
  temperature.as_mut_slice::<F>().copy_from_slice(&[22.1_f64, 24.7, 30.5]);
  values.as_mut_slice::<K>().copy_from_slice(&[time, temperature]);
  flip(new_dictionary(keys, values))
}

#[no_mangle]
pub extern "C" fn create_keyed_table2(dummy: K) -> K{
  enkey(create_table(dummy), 1)
}

#[no_mangle]
pub extern "C" fn keyed_to_simple_table2(dummy: K) -> K{
  unkey(create_keyed_table(dummy))
}

//%% Reference Count %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Example of `decrement_reference_count`.
#[no_mangle]
pub extern "C" fn agriculture(_: K)->K{
  // Produce an apple.
  let fruit=new_symbol("apple");
  // Sow the apple seed.
  decrement_reference_count(fruit);
  // Return null.
  KNULL
}

fn eat(_apple: K){
  println!("おいしい！");
}

/// Example of `increment_reference_count`.
#[no_mangle]
pub extern "C" fn satisfy_5000_men(apple: K) -> K{
  for _ in 0..10{
    eat(apple);
  }
  unsafe{k(0, str_to_S!("eat"), increment_reference_count(apple), KNULL);}
  increment_reference_count(apple)  
}

// %% Callback %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

static mut PIPE:[I; 2]=[-1, -1];

// Callback for some message queue.
extern "C" fn callback(socket: I)->K{
  let mut buffer: [K; 1]=[0 as K];
  unsafe{libc::read(socket, buffer.as_mut_ptr() as *mut V, 8)};
  // Call `shout` function on q side with the received data.
  let result=error_to_string(unsafe{native::k(0, str_to_S!("shout"), buffer[0], KNULL)});
  if result.get_type() == qtype::ERROR{
    eprintln!("Execution error: {}", result.get_symbol().unwrap());
    decrement_reference_count(result);
  };
  KNULL
}

#[no_mangle]
pub extern "C" fn plumber(_: K) -> K{
  if 0 != unsafe{pipe(PIPE.as_mut_ptr())}{
    return new_error("Failed to create pipe\0");
  }
  if KNULL ==register_callback(unsafe{PIPE[0]}, callback){
    return new_error("Failed to register callback\0");
  }
  // Lock symbol in a worker thread.
  pin_symbol();
  let handle=std::thread::spawn(move ||{
    let mut precious=new_simple_list(qtype::SYMBOL, 3);
    let precious_array=precious.as_mut_slice::<S>();
    precious_array[0]=internalize(null_terminated_str_to_S("belief\0"));
    precious_array[1]=internalize(null_terminated_str_to_S("love\0"));
    precious_array[2]=internalize(null_terminated_str_to_S("hope\0"));
    unsafe{libc::write(PIPE[1], std::mem::transmute::<*mut K, *mut V>(&mut precious), 8)};
  });
  handle.join().unwrap();
  unpin_symbol();
  KNULL
}
