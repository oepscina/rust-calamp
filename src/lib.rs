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

#[macro_use]
mod macros;

pub mod message;
pub mod message_header;
pub mod options_header;

pub enum CalAmpError {
    /// Unsupported acknowledgement type.
    AcknowledgementType(u8),

    /// Unsupported encryption type.
    EncryptionType(u8),

    /// Premature end of stream.
    Eos,

    /// Invalid message type.
    MessageType(u8),

    /// Option extension bit length.
    OptionExtensionBitLength(u8),

    /// Invalid service type.
    ServiceType(u8),

    /// Invalid vehicle identification number length.
    VinLength
}
