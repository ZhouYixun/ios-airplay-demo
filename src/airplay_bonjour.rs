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

use mdns_sd::{ServiceDaemon, ServiceInfo};

pub struct AirPlayBonjour {}

impl AirPlayBonjour {
    pub fn new() {
        let mdns = ServiceDaemon::new().expect("Could not create service");
        let airplay_service_type = "_airplay._tcp.local.";
        let airtunes_service_type = "_raop._tcp.local.";
        let instance_name = "Sonic AirPlay V1";
        let my_addrs = "";
        let service_hostname = "sonic.local.";

        let airplay_service_info = ServiceInfo::new(
            &airplay_service_type,
            &instance_name,
            service_hostname,
            my_addrs,
            4459,
            &vec![
                ("deviceid", "73:6f:6e:69:63:00"),
                ("features", "0x5A7FFFF7,0x1E"),
                ("srcvers", "220.68"),
                ("flags", "0x4"),
                ("vv", "2"),
                ("model", "AppleTV2,1"),
                ("rhd", "5.6.0.0"),
                ("pw", "false"),
                ("pk", "b07727d6f6cd6e08b58ede525ec3cdeaa252ad9f683feb212ef8a205246554e7"),
                ("pi", "2e388006-13ba-4041-9a67-25dd4a43d536"),
            ][..],
        )
            .expect("valid service info")
            .enable_addr_auto();

        let airtunes_instance = format!("{}{}", "736f6e696300@", instance_name);
        let airtunes_service_info = ServiceInfo::new(
            &airtunes_service_type,
            &airtunes_instance,
            service_hostname,
            my_addrs,
            4460,
            &vec![
                ("ch", "2"),
                ("cn", "0,1,2,3"),
                ("da", "true"),
                ("et", "0,3,5"),
                ("vv", "2"),
                ("ft", "0x5A7FFFF7,0x1E"),
                ("am", "AppleTV2,1"),
                ("md", "0,1,2"),
                ("rhd", "5.6.0.0"),
                ("pw", "false"),
                ("sr", "44100"),
                ("ss", "16"),
                ("sv", "false"),
                ("tp", "UDP"),
                ("txtvers", "1"),
                ("sf", "0x4"),
                ("vs", "220.68"),
                ("vn", "65537"),
                ("pk", "b07727d6f6cd6e08b58ede525ec3cdeaa252ad9f683feb212ef8a205246554e7"),
            ][..],
        )
            .expect("valid service info")
            .enable_addr_auto();

        let monitor = mdns.monitor().expect("Failed to monitor the daemon");

        mdns.register(airplay_service_info)
            .expect("Failed to register mDNS service");

        println!("Registered service {}.{}", &instance_name, &airplay_service_type);

        mdns.register(airtunes_service_info)
            .expect("Failed to register mDNS service");

        println!("Registered service {}.{}", &instance_name, &airtunes_service_type);

        while let Ok(event) = monitor.recv() {
            println!("Daemon event: {:?}", &event);
        }
    }
}