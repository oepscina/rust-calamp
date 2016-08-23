// +-----------------------------------------------------------------------------------------------+
// | Copyright 2016 Sean Kerr                                                                      |
// |                                                                                               |
// | Licensed under the Apache License, Version 2.0 (the "License");                               |
// | you may not use this file except in compliance with the License.                              |
// | You may obtain a copy of the License at                                                       |
// |                                                                                               |
// |  http://www.apache.org/licenses/LICENSE-2.0                                                   |
// |                                                                                               |
// | Unless required by applicable law or agreed to in writing, software                           |
// | distributed under the License is distributed on an "AS IS" BASIS,                             |
// | WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.                      |
// | See the License for the specific language governing permissions and                           |
// | limitations under the License.                                                                |
// +-----------------------------------------------------------------------------------------------+
// | Author: Sean Kerr <sean@code-box.org>                                                         |
// +-----------------------------------------------------------------------------------------------+

/// Read `$array.len()` bytes from `$slice` into `$array`, and then advance `$index` by
/// `$array.len()` bytes. Upon locating end-of-stream, return prematurely with `CalAmpError::Eos`.
macro_rules! read_into_array {
    ($slice:expr, $index:expr, $array:expr) => ({
        verify_bytes!($slice, $index, $array.len());

        for n in 0..$array.len() {
            $array[n] = read_u8!($slice, $index);
        }
    });
}

/// Read `$length` bytes from `$slice` into a vector, and then advance `$index` by `$length` bytes.
/// Upon locating end-of-stream, return prematurely with `CalAmpError::Eos`.
macro_rules! read_into_vector {
    ($slice:expr, $index:expr, $length:expr, $vector:expr) => ({
        verify_bytes!($slice, $index, $length);

        $vector.extend_from_slice(&$slice[$index..$index+$length]);

        $index += $length;
    });
}

/// Read a u8 from `$slice`, and then advance `$index` by 1 byte. Upon locating end-of-stream,
/// return prematurely with `CalAmpError::Eos`.
macro_rules! read_u8 {
    ($slice:expr, $index:expr) => ({
        verify_bytes!($slice, $index, 1);

        $index += 1;

        $slice[$index - 1]
    });
}

/// Read a u16 from `$slice`, and then advance `$index` by 2 bytes. Upon locating end-of-stream,
/// return prematurely with `CalAmpError::Eos`.
macro_rules! read_u16 {
    ($slice:expr, $index:expr) => ({
        verify_bytes!($slice, $index, 2);

        $index += 2;

        ((($slice[$index - 2] as u16) << 8) + $slice[$index - 1] as u16)
    });
}

/// Read `$length` bytes from `$slice` as a vector, and then advance `$index` by `$length` bytes.
/// Upon locating end-of-stream, return prematurely with `CalAmpError::Eos`.
macro_rules! read_vector {
    ($slice:expr, $index:expr, $length:expr) => ({
        verify_bytes!($slice, $index, $length);

        let mut v = Vec::with_capacity($length);

        v.extend_from_slice(&$slice[$index..$index+$length]);

        $index += $length;

        v
    });
}

/// Verify `$length` bytes are available within `$slice`. Upon locating end-of-stream, return
/// prematurely with `CalAmpError::Eos`.
macro_rules! verify_bytes {
    ($slice:expr, $index:expr, $length:expr) => ({
        if $index + $length >= $slice.len() {
            return Err(CalAmpError::Eos);
        }
    });
}
