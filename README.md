# gm_naughty
Write/Read arbitrary Garry's Mod memory locations.

*Disclaimer: Yes, this is not particularly useful as there better programs that exist to achieve this, however it's Sunday and I'm bored*

## Examples
Getting gmod's base address:
```lua
require("naughty")

print(mem.GetBaseAddress()) -- will return 0x000000000000 if something wrong happens, otherwise gmod's base address
```

Writing to a raw address:
```lua
require("naughty")

local addr = "0x018f2f5292b8"
mem.Write(addr, "wow!") -- writes the bytes for the string starting at the address specified

mem.Write(addr, 1456.56) -- writes the bytes for the number at the address specified

local success, ret = mem.Write(addr, { 45, 52, 100, 75 }) -- writes the bytes in the table at the address specified

if not success then error(ret) end -- mem.Writes returns a bool and the error encountered as a string
```

Writing to an offset:
```lua
require("naughty")

mem.Write(5, { 100, 78, 52 }, true) -- this writes the bytes at the offset specified so => base_addr + 5
```

Reading a raw address:
```lua
require("naughty")

local addr = "0x018f2f5292b8"
local success, bytes = mem.Read(addr, 255) -- 255 is the amount of bytes to read from the address
if not succcess then error(bytes) end

PrintTable(bytes) -- outputs the bytes read
```

Reading at an offset:
```lua
require("naughty")

local success, bytes = mem.Read(5, 255) -- this reads the bytes at the offset specified so => base_addr + 5
if not succcess then error(bytes) end

PrintTable(bytes) -- outputs the bytes read
```

## Compiling
- Open a terminal
- Install **cargo** if you dont have it (on Windows => https://win.rustup.rs) (on Linux/Macos => curl https://sh.rustup.rs -sSf | sh)
- Get [git](https://git-scm.com/downloads) or download the archive for the repository directly
- `git clone https://github.com/Earu/gm_naughty` (ignore this if you've downloaded the archive)
- Run `cd gm_naughty`
- `cargo build`
- Go in `target/debug` and rename the binary according to your branch and realm (gmsv_naughty_win64.dll, gmcl_naughty_win64.dll, gmsv_naughty_linux.dll, gmcl_naughty_linux.dll, gmcl_naughty_osx64.dll)
- Put the binary in your gmod `lua/bin` directory

*Note: Even on other platforms than Windows the extension of your modules **needs** to be **.dll***
