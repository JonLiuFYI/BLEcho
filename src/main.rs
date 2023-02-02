//! Chat client that runs on a PC and connects to BLEcho server
//! Copyright © 2022 JonLiuFYI
//! SPDX-License-Identifier: GPL-3.0-or-later
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::time::Duration;

use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_name = "BLEcho";
    let char_uuid = uuid::uuid!("12570b69-4f11-4b26-bbaa-54127cba01e9");

    let central = {
        let adapters = Manager::new().await?.adapters().await?;
        adapters.into_iter().next().expect("getting central")
    };

    println!("[Client] Looking for server named `{server_name}`");
    central.start_scan(ScanFilter::default()).await?;
    tokio::time::sleep(Duration::from_millis(1500)).await;

    let server = find_server(&central, server_name)
        .await
        .expect("finding server");
    server.connect().await?;
    server.discover_services().await?;
    let characteristics = server.characteristics();
    let chat_ch = characteristics
        .iter()
        .find(|c| c.uuid == char_uuid)
        .expect("getting the chat characteristic");

    println!("[Client] Connected! (Disconnect with Ctrl+D)");

    loop {
        print!("PC: ");
        io::stdout().flush()?;

        let mut msg = String::new();
        let bytecount = io::stdin().read_line(&mut msg)?;
        // Leave loop with EOF (Ctrl+D)
        if bytecount == 0 {
            break;
        }

        server
            .write(chat_ch, msg.as_bytes(), WriteType::WithoutResponse)
            .await?;
    }

    println!("\n[Client] Disconnecting from `{server_name}`");
    server.disconnect().await?;
    println!("[Client] Disconnected");

    Ok(())
}

async fn find_server(central: &Adapter, server_name: &str) -> Option<Peripheral> {
    let peripherals = central.peripherals().await.ok()?;
    for p in peripherals {
        let p_name = {
            let p_props = p.properties().await.expect("getting properties")?;
            p_props.local_name?
        };
        if p_name.eq(server_name) {
            return Some(p);
        }
    }

    None
}
