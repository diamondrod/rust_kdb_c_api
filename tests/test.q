/
* @file test.q
* @overview Tests of C API examples. The artefact of `c_api_examples` is loaded
* and functions are called from q side.
\

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                           Inital Setting     			                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// Fetch shared object from build directory.
//system "cp ../target/debug/libc_api_examples.so .";

// Load test helper functions.
\l test_helper_function.q

// Function to load shared library.
LIBPATH_: `libc_api_examples 2:

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	Load Libraries     			                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// These function list can be checked against `nm -D libc_api_examples.so | awk '$2 ~/T/ {print $3}'`.

// decrement_reference_count
.capi.agriculture: LIBPATH_ (`agriculture; 1);
// ee
.capi.catchy: LIBPATH_ (`catchy; 2);
// jv
.capi.concat_list: LIBPATH_ (`concat_list; 2);
// b9
.capi.conceal: LIBPATH_ (`conceal; 1);
// append
.capi.concat_list2: LIBPATH_ (`concat_list2; 2);
// kb
.capi.create_bool: LIBPATH_ (`create_bool; 1);
// kb
.capi.create_bool2: LIBPATH_ (`create_bool2; 1);
// kg
.capi.create_byte: LIBPATH_ (`create_byte; 1);
// ku
.capi.create_guid: LIBPATH_ (`create_guid; 1);
// ki
.capi.create_int: LIBPATH_ (`create_int; 1);
// kj
.capi.create_long: LIBPATH_ (`create_long; 1);
// kc
.capi.create_char: LIBPATH_ (`create_char; 1);
// new_char
.capi.create_char2: LIBPATH_ (`create_char2; 1);
// jk
.capi.create_compound_list: LIBPATH_ (`create_compound_list; 1);
// push
.capi.create_compound_list2: LIBPATH_ (`create_compound_list2; 1);
// kd
.capi.create_date: LIBPATH_ (`create_date; 1);
// kz
.capi.create_datetime: LIBPATH_ (`create_datetime; 1);
// xD
.capi.create_dictionary: LIBPATH_ (`create_dictionary; 1);
// kf
.capi.create_float: LIBPATH_ (`create_float; 1);
// knt
.capi.create_keyed_table: LIBPATH_ (`create_keyed_table; 1);
// enkey
.capi.create_keyed_table2: LIBPATH_ (`create_keyed_table2; 1);
// new_minute
.capi.create_minute: LIBPATH_ (`create_minute; 1);
// new_month
.capi.create_month: LIBPATH_ (`create_month; 1);
// ke
.capi.create_real: LIBPATH_ (`create_real; 1);
// kh
.capi.create_short: LIBPATH_ (`create_short; 1);
// new_second
.capi.create_second: LIBPATH_ (`create_second; 1);
// ja
.capi.create_simple_list: LIBPATH_ (`create_simple_list; 1);
// push_raw
.capi.create_simple_list2: LIBPATH_ (`create_simple_list2; 1);
// kp
.capi.create_string: LIBPATH_ (`create_string; 1);
// kpn
.capi.create_string2: LIBPATH_ (`create_string2; 1);
// ks
.capi.create_symbol: LIBPATH_ (`create_symbol; 1);
// new_symbol
.capi.create_symbol2: LIBPATH_ (`create_symbol2; 1);
// js
.capi.create_symbol_list: LIBPATH_ (`create_symbol_list; 1);
// push_symbol
.capi.create_symbol_list2: LIBPATH_ (`create_symbol_list2; 1);
// xT
.capi.create_table: LIBPATH_ (`create_table; 1);
// flip
.capi.create_table2: LIBPATH_ (`create_table2; 1);
// kt
.capi.create_time: LIBPATH_ (`create_time; 1);
// ktj
.capi.create_timespan: LIBPATH_ (`create_timespan; 1);
// new_timespan
.capi.create_timespan2: LIBPATH_ (`create_timespan2; 1);
// ktj
.capi.create_timestamp: LIBPATH_ (`create_timestamp; 1);
// new_timestamp
.capi.create_timestamp2: LIBPATH_ (`create_timestamp2; 1);
// dj
.capi.days_to_date: LIBPATH_ (`days_to_date; 1);
// q_ipc_decode
.capi.decrypt: LIBPATH_ (`decrypt; 1);
// k
.capi.dictionary_list_to_table: LIBPATH_ (`dictionary_list_to_table; 1);
// q_ipc_encode
.capi.encrypt: LIBPATH_ (`encrypt; 1);
// r0
.capi.idle_man: LIBPATH_ (`idle_man; 1);
// new_error
.capi.keep_out: LIBPATH_ (`keep_out; 1);
// ktd
.capi.keyed_to_simple_table: LIBPATH_ (`keyed_to_simple_table; 1);
// unkey
.capi.keyed_to_simple_table2: LIBPATH_ (`keyed_to_simple_table2; 1);
// as_mut_slice
.capi.modify_long_list_a_bit: LIBPATH_ (`modify_long_list_a_bit; 1);
// str_to_const_S
.capi.must_be_int: LIBPATH_ (`must_be_int; 1);
// error_to_string
.capi.no_panick: LIBPATH_ (`no_panick; 2);
// setm
.capi.parallel_sym_change: LIBPATH_ (`parallel_sym_change; 1);
// r1
.capi.pass_through_cave: LIBPATH_ (`pass_through_cave; 1);
// str_to_S
.capi.pingpong: LIBPATH_ (`pingpong; 1);
// null_terminated_str_to_S
.capi.pingpong2: LIBPATH_ (`pingpong2; 1);
// get_bool
.capi.print_bool: LIBPATH_ (`print_bool; 1);
// get_byte
.capi.print_byte: LIBPATH_ (`print_byte; 1);
// get_char
.capi.print_char: LIBPATH_ (`print_char; 1);
// get_float
.capi.print_float: LIBPATH_ (`print_float; 1);
// get_guid
.capi.print_guid: LIBPATH_ (`print_guid; 1);
// get_int
.capi.print_int: LIBPATH_ (`print_int; 1);
// get_long
.capi.print_long: LIBPATH_ (`print_long; 1);
// get_real
.capi.print_real: LIBPATH_ (`print_real; 1);
// get_short
.capi.print_short: LIBPATH_ (`print_short; 1);
// get_str
.capi.print_string: LIBPATH_ (`print_string; 1);
// get_string
.capi.print_string2: LIBPATH_ (`print_string2; 1);
// S_to_str
.capi.print_symbol: LIBPATH_ (`print_symbol; 1);
// get_symbol
.capi.print_symbol2: LIBPATH_ (`print_symbol2; 1);
// register_callback
.capi.plumber: LIBPATH_ (`plumber; 1);
// d9
.capi.reveal: LIBPATH_ (`reveal; 1);
// dot
.capi.rust_parse: LIBPATH_ (`rust_parse; 2);
// increment_reference_count
.capi.satisfy_5000_men: LIBPATH_ (`satisfy_5000_men; 1);
// krr
.capi.thai_kick: LIBPATH_ (`thai_kick; 1);
// KNULL
.capi.vanity: LIBPATH_ (`vanity; 1);
// ymd
.capi.ymd_to_days: LIBPATH_ (`ymd_to_days; 1);

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	  Tests    	        		                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Global Variable %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// KNULL
.test.ASSERT_EQ["KNULL"; .capi.vanity[]; (::)]

//%% Macros %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// str_to_S
ping:{[int] `$string[int], "_pong!!"};
.test.ASSERT_EQ["str_to_S"; .capi.pingpong[]; `$"77_pong!!"]

//%% KUtility %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// as_mut_slice
// Assign to a variable to keep the result.
.test.ASSERT_EQ["as_mut_slice - success"; .capi.modify_long_list_a_bit[list:1 2 3]; 1 30000 3]
// as_mut_slice (return error)
.test.ASSERT_ERROR["as_mut_slice - failure"; .capi.modify_long_list_a_bit; enlist enlist 1; "this list is not long enough"]

// get_bool
.test.ASSERT_EQ["get_bool - true"; .capi.print_bool[1b]; (::)]
// get_bool
.test.ASSERT_EQ["get_bool - false"; .capi.print_bool[0b]; (::)]
// get_bool - failure
.test.ASSERT_ERROR["get_bool - failure"; .capi.print_bool; enlist 100; "not a bool"]

// get_byte
.test.ASSERT_EQ["get_byte"; .capi.print_byte[0xc4]; (::)]
// get_byte - failure
.test.ASSERT_ERROR["get_byte - failure"; .capi.print_byte; enlist "c"; "not a byte"]

// get_guid
guid: first 1?0Ng;
.test.ASSERT_EQ["get_guid"; .capi.print_guid[guid]; (::)]
// get_guid - failure
.test.ASSERT_ERROR["get_guid - failure"; .capi.print_guid; enlist 0x7a; "not a GUID"]

// get_short
.test.ASSERT_EQ["get_short"; .capi.print_short[10h]; (::)]
// get_short - failure
.test.ASSERT_ERROR["get_short - failure"; .capi.print_short; enlist 10; "not a short"]

// get_int
.test.ASSERT_EQ["get_int"; .capi.print_int[42i]; (::)]
// get_int - month
.test.ASSERT_EQ["get_int - month"; .capi.print_int[2010.03m]; (::)]
// get_int - date
.test.ASSERT_EQ["get_int - date"; .capi.print_int[2020.02.01]; (::)]
// get_int - minute
.test.ASSERT_EQ["get_int - minute"; .capi.print_int[12:03]; (::)]
// get_int - second
.test.ASSERT_EQ["get_int - second"; .capi.print_int[03:57:20]; (::)]
// get_int - time
.test.ASSERT_EQ["get_int - time"; .capi.print_int[00:34:16.636]; (::)]
// get_int - error
.test.ASSERT_ERROR["get_int - failure1"; .capi.print_int; enlist `error; "not an int"]
// get_int - error
.test.ASSERT_ERROR["get_int - failure2"; .capi.print_int; enlist 10000; "not an int"]

// get_long
.test.ASSERT_EQ["get_long"; .capi.print_long[-109210]; (::)]
// get_long - timestamp
.test.ASSERT_EQ["get_long - timestamp"; .capi.print_long[2000.01.01D12:00:00.123456789]; (::)]
// get_long - timespan
.test.ASSERT_EQ["get_long - timespan"; .capi.print_long[-3D18:23:09.000000021]; (::)]
// get_long - error
.test.ASSERT_ERROR["get_long - failure"; .capi.print_long; enlist 1b; "not a long"]

// get_real
.test.ASSERT_EQ["get_real"; .capi.print_real[193810.32e]; (::)]
// get_real - error
.test.ASSERT_ERROR["get_real - failure"; .capi.print_real; enlist 100f; "not a real"]

// get_float
.test.ASSERT_EQ["get_float"; .capi.print_float[-37017.0933]; (::)]
// get_float - datetime
.test.ASSERT_EQ["get_float - datetime"; .capi.print_float[2002.01.12T10:03:45.332]; (::)]
// get_float - error
.test.ASSERT_ERROR["get_float - failure"; .capi.print_float; enlist .z.p; "not a float"]

// get_char
.test.ASSERT_EQ["get_char"; .capi.print_char["k"]; (::)]
// get_char - error
.test.ASSERT_ERROR["get_char - failure1"; .capi.print_char; enlist "devour"; "not a char"]
// get_char - error
.test.ASSERT_ERROR["get_char - failure2"; .capi.print_char; enlist 1b; "not a char"]

// get_symbol
.test.ASSERT_EQ["get_symbol"; .capi.print_symbol2[`locust]; (::)]
// get_symool - error
.test.ASSERT_ERROR["get_symbol - failure"; .capi.print_symbol2; enlist "attack!"; "not a symbol"]

// get_str
.test.ASSERT_EQ["get_str"; .capi.print_string["gnat"]; (::)]

// get_string
.test.ASSERT_EQ["get_string"; .capi.print_string2["grasshopper"]; (::)]
// get_string - error
.test.ASSERT_ERROR["get_string - failure"; .capi.print_string2; enlist (1 2; `a`b); "not a string"]

// append
.test.ASSERT_EQ["append - compound"; .capi.concat_list2[(::; `metals; `fire); ("clay"; 316)]; (::; `metals; `fire; "clay"; 316)]
.test.ASSERT_EQ["append - long"; .capi.concat_list2[1 2 3; 4 5]; 1 2 3 4 5]
.test.ASSERT_EQ["append - symbol"; .capi.concat_list2[`a`b`c; `d`e]; `a`b`c`d`e]
// append - error
.test.ASSERT_ERROR["append - failure"; .capi.concat_list2; (1 2 3; "45"); "not a list or types do not match"]

// push
.test.ASSERT_EQ["push"; .capi.create_compound_list2[5i]; (til 5), 5i]

// push_raw
.test.ASSERT_EQ["push_raw"; .capi.create_simple_list2[]; 2000.01.01+til 5]

// push_symbol
.test.ASSERT_EQ["push_symbol"; .capi.create_symbol_list2[]; `Abraham`Isaac`Jacob`Joseph]

// q_ipc_encode
list: (til 3; "abc"; 2018.02.18D04:30:00.000000000; `revive);
.test.ASSERT_EQ["q_ipc_encode"; .capi.encrypt[list]; bytes:-8!list]

// q_ipc_encode - compress
long_list: 1000#/: ("long"; `list);
.test.ASSERT_EQ["q_ipc_encode - compress"; .capi.encrypt[long_list]; long_bytes:-8!long_list]

// q_ipc_decode
.test.ASSERT_EQ["q_ipc_decode"; .capi.decrypt[bytes]; list]

// q_ipc_decode - decompress
.test.ASSERT_EQ["q_ipc_decode - decompress"; .capi.decrypt[long_bytes]; long_list]

// q_ipc_decode - failure
.test.ASSERT_ERROR["q_ipc_decode - failure"; .capi.decrypt; enlist `hello; "not bytes"]

// q_ipc_decode - failure2
.test.ASSERT_ERROR["q_ipc_decode - failure2"; .capi.decrypt; enlist 0x00aa12345678; "failed to decode"]

//%% Constructors %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// kb
.test.ASSERT_EQ["kb"; .capi.create_bool[]; 1b]

// ku
.test.ASSERT_EQ["ku"; .capi.create_guid[]; "G"$"1e11170c-4224-252c-1c14-1e224d3d4624"]

// kg
.test.ASSERT_EQ["kg"; .capi.create_byte[]; 0x3c]

// kh
.test.ASSERT_EQ["kh"; .capi.create_short[]; -144h]

// ki
.test.ASSERT_EQ["ki"; .capi.create_int[]; 86400000i]

// kj
.test.ASSERT_EQ["kj"; .capi.create_long[]; -668541276001729000]

// ke
.test.ASSERT_EQ["ke"; .capi.create_real[]; 0.00324e]

// kf
.test.ASSERT_EQ["kf"; .capi.create_float[]; -6302.620]

// kc
.test.ASSERT_EQ["kc"; .capi.create_char[]; "q"]

// ks
.test.ASSERT_EQ["ks"; .capi.create_symbol[]; `symbolism]

// ktj - timestamp
.test.ASSERT_EQ["ktj - timestamp"; .capi.create_timestamp[]; 2015.03.16D00:00:00:00.000000000]

// ktj - timespan
.test.ASSERT_EQ["ktj - timespan"; .capi.create_timespan[]; -1D01:30:00.001234567]

// kd
.test.ASSERT_EQ["kd"; .capi.create_date[]; 1999.12.25]

// kz
.test.ASSERT_EQ["kz"; .capi.create_datetime[]; 2015.03.16T12:00:00:00]

// kt
.test.ASSERT_EQ["kz"; .capi.create_time[]; -01:30:00.123]

// kp
.test.ASSERT_EQ["kp"; .capi.create_string[]; "this is a text."]

// kpn
.test.ASSERT_EQ["kpn"; .capi.create_string2[]; "The meeting was too long"]

// xT
.test.ASSERT_EQ["xT"; .capi.create_table[]; table:([] time: 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392; temperature: 22.1, 24.7, 30.5)]

// ktd
.test.ASSERT_EQ["xT"; .capi.keyed_to_simple_table[]; table]

// xD
.test.ASSERT_EQ["xD"; .capi.create_dictionary[]; 0 1i!(2000.01.01 2000.01.02 2000.01.03; "I'm afraid I would crash the application...")]

// knt
.test.ASSERT_EQ["xT"; .capi.create_keyed_table[]; 1!table]

// krr
.test.ASSERT_ERROR["krr"; .capi.thai_kick; enlist (::); "Thai kick unconditionally!!"]

// ja
.test.ASSERT_EQ["ja"; .capi.create_simple_list[]; 2000.01.01D00:00:00 2000.01.02D00:00:00 2000.01.03D00:00:00 2000.01.04D00:00:00 2000.01.05D00:00:00 ]

// jv
.test.ASSERT_EQ["jv - compound"; .capi.concat_list[(::; `metals; `fire); ("clay"; 316)]; (::; `metals; `fire; "clay"; 316)]
.test.ASSERT_EQ["jv - long"; .capi.concat_list[1 2 3; 4 5]; 1 2 3 4 5]
.test.ASSERT_EQ["jv - symbol"; .capi.concat_list[`a`b`c; `d`e]; `a`b`c`d`e]

// jk
.test.ASSERT_EQ["jk"; .capi.create_compound_list[]; (`1st; 2i; "3rd")]

// js
.test.ASSERT_EQ["js"; .capi.create_symbol_list[]; `Abraham`Isaac`Jacob`Joseph]

// ee
.test.ASSERT_EQ["ee - success"; .capi.catchy[$; ("S"; "rust")]; `rust]
// ee (print error to stdout)
.test.ASSERT_EQ["ee - failure"; .capi.catchy[+; (2; "rust")]; (::)]

//%% IPC Functions %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// k
.test.ASSERT_EQ[enlist "k"; .capi.dictionary_list_to_table[]; ([] a: 0 10 20i; b: 0 100 200i)]

// b9
.test.ASSERT_EQ["b9"; .capi.conceal["Look! HE has come!!"]; -8!"Look! HE has come!!"]

// d9
.test.ASSERT_EQ["d9"; .capi.reveal[-8!(`contact`from; "space"; 12)]; (`contact`from; "space"; 12)]

//%% Reference count %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// r0
.test.ASSERT_EQ["r0"; .capi.idle_man[]; (::)]

// r1
get_item1:{[man] "a basket of summer fruit"};
get_item2:{[man] "boiling pot, facing away from the north"}
.test.ASSERT_EQ["r1"; .capi.pass_through_cave[`son_of_man]; `son_of_man]

//%% Miscellaneous %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// dot
.test.ASSERT_EQ["dot"; .capi.rust_parse[$; ("J"; "42")]; 42]

// setm
.test.ASSERT_EQ["dot"; .capi.parallel_sym_change[`a`b]; `replaced`symbolbol]

// ymd
.test.ASSERT_EQ["ymd"; .capi.ymd_to_days[]; 7396i]

// dj
.test.ASSERT_EQ["dj"; .capi.days_to_date[7396i]; 20200401i]

//%% Utility Functions %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// S_to_str (print value to stdout)
.test.ASSERT_EQ["S_to_str"; .capi.print_symbol[`rust]; (::)]

// null_terminated_str_to_S
.test.ASSERT_EQ["null_terminated_str_to_S"; .capi.pingpong2[]; `$"77_pong!!"]

// null_terminated_str_to_const_S
.test.ASSERT_ERROR["str_to_const_S"; .capi.must_be_int; enlist 10000; "not an int"]

//%% Re-Export %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// new_char
.test.ASSERT_EQ["new_char"; .capi.create_char2[]; "t"]

// new_symbol
.test.ASSERT_EQ["new_symbol"; .capi.create_symbol2[]; `symbolic]

// new_timestamp
.test.ASSERT_EQ["new_timestamp"; .capi.create_timestamp2[]; 2015.03.16D00:00:00:00.000000000]

// new_month
.test.ASSERT_EQ["new_month"; .capi.create_month[]; 2010.07m]

// new_timespan
.test.ASSERT_EQ["new_timespan"; .capi.create_timespan2[]; -1D01:30:00.001234567]

// new_minute
.test.ASSERT_EQ["new_minute"; .capi.create_minute[]; 10:40]

// new_second
.test.ASSERT_EQ["new_second"; .capi.create_second[]; -02:00:00]

// new_error
.test.ASSERT_ERROR["new_error"; .capi.keep_out; enlist (::); "No means no"]

// error_to_string
.test.ASSERT_EQ["error_to_string"; .capi.no_panick[sum; enlist til 10]; 45]
// error_to_string -failure
.test.ASSERT_EQ["error_to_string - failure"; .capi.no_panick[sum; enlist `cannot`add`symbol]; (::)]

// flip
.test.ASSERT_EQ["flip"; .capi.create_table2[]; table:([] time: 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392; temperature: 22.1, 24.7, 30.5)]

// unkey
.test.ASSERT_EQ["unkey"; .capi.keyed_to_simple_table2[]; table]

// key
.test.ASSERT_EQ["enkey"; .capi.create_keyed_table2[]; 1!table]

// decrement_reference_count
.test.ASSERT_EQ["decrement_eference_count"; .capi.agriculture[]; (::)]

// increment_reference_count
eat:{[apple] show "Collect the clutter of apples!";};
.test.ASSERT_EQ["increment_eference_count"; .capi.satisfy_5000_men[`green_apple]; `green_apple]

// register_callback
shout:{[precious] -1 "What are the three largest elements?: ", .Q.s1 precious;};
.test.ASSERT_EQ["register_callback"; .capi.plumber[]; (::)]

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	  Result   	        		                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// Show result.
.test.DISPLAY_RESULT[]
