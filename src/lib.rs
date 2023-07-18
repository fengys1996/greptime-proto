// Copyright 2023 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod serde;
pub mod v1;

pub mod prometheus {
    pub mod remote {
        tonic::include_proto!("prometheus");
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use prost::Message;

    use crate::v1::meta::{HeartbeatRequest, RegionStat, TableName};

    #[test]
    fn test_payload_of_heartbeat() {
        print_payload_size_of_heartbeat(100);
        print_payload_size_of_heartbeat(1000);
        print_payload_size_of_heartbeat(10000);
        print_payload_size_of_heartbeat(100000);
    }

    fn print_payload_size_of_heartbeat(region_num: usize) {
        let req = HeartbeatRequest {
            region_stats: mock_region_stats(region_num),
            ..Default::default()
        };

        println!(
            "The size of payload: {} KB, Region number: {}, ",
            req.encode_to_vec().len() as f32 / 1024.0,
            region_num,
        );
    }

    fn mock_region_stats(region_num: usize) -> Vec<RegionStat> {
        let mut v = Vec::with_capacity(region_num);
        for _ in 0..region_num {
            v.push(mock_region_stat());
        }
        v
    }

    fn mock_region_stat() -> RegionStat {
        let table_name = TableName {
            catalog_name: "1234567890123".to_string(),
            schema_name: "1234567890123".to_string(),
            table_name: "1234567890123".to_string(),
        };
        RegionStat {
            region_id: 1,
            table_name: Some(table_name),
            rcus: 1000000,
            wcus: 1000000,
            approximate_bytes: 1000000,
            approximate_rows: 1000000,
            attrs: HashMap::new(),
        }
    }
}
