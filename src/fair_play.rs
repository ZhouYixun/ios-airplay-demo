/*
    sonic-ios-airplay is intended to easily create AirPlay server acting like Apple TV and receive it.
    Copyright (C) 2023 SonicCloudOrg

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::io::{Read, Write};
use std::net::TcpStream;
use log::info;

pub struct FairPlay {
    key_msg: [u8; 164],
}

impl FairPlay {
    fn set_up(&self, stream: &mut TcpStream) -> std::io::Result<usize> {
        let mut buf = [0; 128];
        loop {
            let len = stream.read(&mut buf).unwrap();
            if len == 0 {
                info!("read done.");
                break;
            }
        }
        if buf[4] != 3 {
            info!("version {} is not supported.", buf[4]);
            Ok(())
        }
        if buf.len() == 16 {
            let mode = buf[14];
            let reply_msg: Vec<Vec<u8>> = vec![
                vec![70, 80, 76, 89, 3, 1, 2, 0, 0, 0, 0, -126, 2, 0, 15, -97, 63, -98, 10, 37, 33, -37, -33, 49, 42, -78, -65, -78, -98, -115, 35, 43, 99, 118, -88, -56, 24, 112, 29, 34, -82, -109, -40, 39, 55, -2, -81, -99, -76, -3, -12, 28, 45, -70, -99, 31, 73, -54, -86, -65, 101, -111, -84, 31, 123, -58, -9, -32, 102, 61, 33, -81, -32, 21, 101, -107, 62, -85, -127, -12, 24, -50, -19, 9, 90, -37, 124, 61, 14, 37, 73, 9, -89, -104, 49, -44, -100, 57, -126, -105, 52, 52, -6, -53, 66, -58, 58, 28, -39, 17, -90, -2, -108, 26, -118, 109, 74, 116, 59, 70, -61, -89, 100, -98, 68, -57, -119, 85, -28, -99, -127, 85, 0, -107, 73, -60, -30, -9, -93, -10, -43, -70],
                vec![70, 80, 76, 89, 3, 1, 2, 0, 0, 0, 0, -126, 2, 1, -49, 50, -94, 87, 20, -78, 82, 79, -118, -96, -83, 122, -15, 100, -29, 123, -49, 68, 36, -30, 0, 4, 126, -4, 10, -42, 122, -4, -39, 93, -19, 28, 39, 48, -69, 89, 27, -106, 46, -42, 58, -100, 77, -19, -120, -70, -113, -57, -115, -26, 77, -111, -52, -3, 92, 123, 86, -38, -120, -29, 31, 92, -50, -81, -57, 67, 25, -107, -96, 22, 101, -91, 78, 25, 57, -46, 91, -108, -37, 100, -71, -28, 93, -115, 6, 62, 30, 106, -16, 126, -106, 86, 22, 43, 14, -6, 64, 66, 117, -22, 90, 68, -39, 89, 28, 114, 86, -71, -5, -26, 81, 56, -104, -72, 2, 39, 114, 25, -120, 87, 22, 80, -108, 42, -39, 70, 104, -118],
                vec![70, 80, 76, 89, 3, 1, 2, 0, 0, 0, 0, -126, 2, 2, -63, 105, -93, 82, -18, -19, 53, -79, -116, -35, -100, 88, -42, 79, 22, -63, 81, -102, -119, -21, 83, 23, -67, 13, 67, 54, -51, 104, -10, 56, -1, -99, 1, 106, 91, 82, -73, -6, -110, 22, -78, -74, 84, -126, -57, -124, 68, 17, -127, 33, -94, -57, -2, -40, 61, -73, 17, -98, -111, -126, -86, -41, -47, -116, 112, 99, -30, -92, 87, 85, 89, 16, -81, -98, 14, -4, 118, 52, 125, 22, 64, 67, -128, 127, 88, 30, -28, -5, -28, 44, -87, -34, -36, 27, 94, -78, -93, -86, 61, 46, -51, 89, -25, -18, -25, 11, 54, 41, -14, 42, -3, 22, 29, -121, 115, 83, -35, -71, -102, -36, -114, 7, 0, 110, 86, -8, 80, -50],
                vec![70, 80, 76, 89, 3, 1, 2, 0, 0, 0, 0, -126, 2, 3, -112, 1, -31, 114, 126, 15, 87, -7, -11, -120, 13, -79, 4, -90, 37, 122, 35, -11, -49, -1, 26, -69, -31, -23, 48, 69, 37, 26, -5, -105, -21, -97, -64, 1, 30, -66, 15, 58, -127, -33, 91, 105, 29, 118, -84, -78, -9, -91, -57, 8, -29, -45, 40, -11, 107, -77, -99, -67, -27, -14, -100, -118, 23, -12, -127, 72, 126, 58, -24, 99, -58, 120, 50, 84, 34, -26, -9, -114, 22, 109, 24, -86, 127, -42, 54, 37, -117, -50, 40, 114, 111, 102, 31, 115, -120, -109, -50, 68, 49, 30, 75, -26, -64, 83, 81, -109, -27, -17, 114, -24, 104, 98, 51, 114, -100, 34, 125, -126, 12, -103, -108, 69, -40, -110, 70, -56, -61, 89],
            ];
            stream.write(reply_msg[mode])
        } else if buf.len() == 164 {
            &self.key_msg.copy_from_slice(&*buf);
            let fp_header: [u8; 12] = [70, 80, 76, 89, 3, 1, 4, 0, 0, 0, 0, 20];
            stream.write(&*fp_header).expect("TODO: panic message");
            stream.write(&self.key_msg)
        }
    }
}