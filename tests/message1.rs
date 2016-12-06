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

extern crate calamp;

use std::fs::File;
use std::io::prelude::*;

use calamp::message_header::*;
use calamp::options_header::*;

#[test]
fn message1() {
    let mut v = Vec::new();

    File::open("tests/sample/message1.bin").unwrap()
                                           .read_to_end(&mut v)
                                           .unwrap();

    let mut byte_count = match OptionsHeader::parse(&v) {
        Ok((options, byte_count)) => {
            match options.mobile_id() {
                &Some(ref id) => println!("{:?}", id),
                _ => panic!("OptionsHeader::mobile_id is empty")
            }

            match options.authentication() {
                &Some(ref authentication) => println!("Authentication: {:?}", &authentication),
                &None => println!("Authentication: None")
            }

            match options.extension() {
                &Some(ref extension) => {
                    match extension.encryption_service() {
                        &Some(ref _data) => {
                        },
                        &None => {
                        }
                    }

                    match extension.esn() {
                        &Some(ref esn) => println!("Extension ESN: {}", esn),
                        &None => println!("Extension ESN: None")
                    }

                    match extension.vin() {
                        &Some(ref vin) => println!("Extension VIN: {}", vin),
                        &None => println!("Extension VIN: None")
                    }
                },
                &None => println!("Extension: None")
            }

            match options.forwarding() {
                &Some((ref ip, ref port, ref protocol, ref op)) => {
                    println!("Forwarding: {}:{} {:?} {:?}", ip, port, protocol, op);
                },
                &None => println!("Forwarding: None")
            }

            match options.redirection() {
                &Some((ref ip, ref port)) => println!("Redirection: {}:{}", ip, port),
                &None => println!("Redirection: None")
            }

            match options.routing() {
                &Some(ref routing) => println!("Routing: {:?}", &routing),
                &None => println!("Routing: None")
            }

            byte_count
        },
        _ => panic!("Failed to parse OptionsHeader")
    };

    byte_count = match MessageHeader::parse(&v[byte_count..]) {
        Ok((message, byte_count)) => {
            println!("{:?}", message.service_type());
            println!("{:?}", message.message_type());
            println!("Sequence Number: {}", message.sequence_number());
            byte_count
        },
        _ => panic!("Failed to parse MessageHeader")
    };
}
