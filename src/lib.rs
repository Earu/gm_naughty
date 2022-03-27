#![feature(c_unwind)]

use vmemory::ProcessMemory;
use sysinfo;

#[macro_use]
extern crate gmod;

fn try_get_memory_range() -> Option<ProcessMemory> {
	match sysinfo::get_current_pid() {
		Ok(pid) => {
			match ProcessMemory::attach_process(pid as u32) {
				Some(mem) => {
					mem.resume();
					Some(mem)
				},
				None => None,
			}
		},
		Err(_) => None
	}
}

fn try_into_u8(val: isize) -> Option<u8> {
	if val < 0 || val > 255 {
		None
	} else {
		match u8::try_from(val as usize) {
			Ok(val) => Some(val),
			Err(_) => None
		}
	}
}

unsafe fn deny(lua: gmod::lua::State, msg: &str) -> i32 {
	lua.push_boolean(false);
	lua.push_string(msg);
	2
}

#[lua_function]
unsafe fn write_mem(lua: gmod::lua::State) -> i32 {
	match try_get_memory_range() {
		Some(mem) => {
			// we're using a string here because with precision error addresses could get truncated
			let addr = match lua.check_string(1).parse::<usize>() {
				Ok(addr) => addr,
				Err(_) => return deny(lua, "Invalid address"),
			};

			let should_offset = lua.get_boolean(3); // if true, we'll offset the address by the base address of the memory range
			let data: Vec<u8> = match lua.get_type(2) {
				"number" => {
					let value = lua.check_number(2);
					value.to_ne_bytes().to_vec()
				},
				"string" => {
					let value = lua.check_string(2);
					value.as_bytes().to_vec()
				},
				"table" => {
					let mut value = Vec::new();
					while lua.next(2) != 0 {
						match try_into_u8(lua.check_integer(-1)) {
							Some(byte) => {
								value.push(byte)
							},
							None => return deny(lua, "Invalid byte value"),
						}

						lua.pop();
					}
					value
				},
				_ => return deny(lua, "Unsupported data type"),
			};

			mem.write_memory(addr, &data, should_offset);

			lua.push_boolean(true);
			1
		},
		None => deny(lua, "Could not get memory range of process"),
	}
}

#[lua_function]
unsafe fn read_mem(lua: gmod::lua::State) -> i32 {
	match try_get_memory_range() {
		Some(mem) => {
			// we're using a string here because with precision error addresses could get truncated
			let addr = match lua.check_string(1).parse::<usize>() {
				Ok(addr) => addr,
				Err(_) => return deny(lua, "Invalid address"),
			};

			let requested_size = lua.check_number(2);
			if requested_size < 0.0 {
				return deny(lua, "Invalid size");
			}

			let size = match usize::try_from(lua.check_number(2) as u64) {
				Ok(size) => size,
				Err(_) => return deny(lua, "Invalid size"),
			};

			// if true, we'll offset the address by the base address of the memory range
			let should_offset = lua.get_boolean(3);
			mem.read_memory(addr, size, should_offset);

			lua.push_boolean(true);
			1
		},
		None => deny(lua, "Could not get memory range of process"),
	}
}

#[lua_function]
unsafe fn get_base_addr(lua: gmod::lua::State) -> i32 {
	match try_get_memory_range() {
		Some(mem) => {
			let hex = format!("{:x}", mem.base());
			lua.push_string(&hex);
		},
		None => lua.push_string("0x00000000"),
	}

	1
}

#[gmod13_open]
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
	lua.new_table();

	lua.push_function(write_mem);
	lua.set_field(-2, lua_string!("Write"));

	lua.push_function(read_mem);
	lua.set_field(-2, lua_string!("Read"));

	lua.push_function(get_base_addr);
	lua.set_field(-2, lua_string!("GetBaseAddress"));

	lua.set_global(lua_string!("mem"));

    0
}

#[gmod13_close]
unsafe fn gmod13_close(_: gmod::lua::State) -> i32 {
    0
}