use std::collections::HashMap;

use anyhow::bail;
use futures_lite::StreamExt;

pub async fn mount(device: &str) -> anyhow::Result<String> {
    let client = udisks2::Client::new().await?;

    let object = client
        .object(format!("/org/freedesktop/UDisks2/block_devices/{device}"))
        .expect("No {device} device found");
    let filesystem = object.filesystem().await?;
    let options = HashMap::new();
    let mount = filesystem.mount(options).await?;
    Ok(mount)
}

pub async fn unmount(device: &str) -> anyhow::Result<()> {
    let client = udisks2::Client::new().await?;

    let object = client
        .object(format!("/org/freedesktop/UDisks2/block_devices/{device}"))
        .expect("No {device} device found");
    let filesystem = object.filesystem().await?;
    let options = HashMap::new();
    let unmount = filesystem.unmount(options).await?;
    Ok(())
}

/// Returns something like "/org/freedesktop/UDisks2/block_devices/sda"
pub async fn wait_for_device() -> anyhow::Result<String> {
    let client = udisks2::Client::new().await?;
    let mut stream = client.object_manager().receive_interfaces_added().await?;

    // ASCII for "/org/freedesktop/UDisks2/block_devices/"
    const BLOCK_DEVICE: [u8; 39] = [47u8,111,114,103,47,102,114,101,101,100,101,115,107,116,111,112,47,85,68,105,115,107,115,50,47,98,108,111,99,107,95,100,101,118,105,99,101,115,47];

    while let a_result = stream.next().await {
        if let Some(interface_added) = a_result {
            let data = interface_added.message().data().bytes();
            let max = data.len() - BLOCK_DEVICE.len();

            'outer: for i in 0..=max {
                let delta_data = &data[i..];

                let same = delta_data.iter()
                    .zip(BLOCK_DEVICE)
                    .fold(true, |acc, (left, right)| acc && *left == right);

                if same {
                    let result: Vec<u8> = delta_data.iter()
                        .take_while(|a_byte| **a_byte != 0)
                        .map(|a_byte| *a_byte)
                        .collect();

                    if let Some(charachter) = result.last() {
                        const ZERO: u8 = 0x30;
                        const NINE: u8 = 0x39;

                        if !(ZERO..=NINE).contains(charachter) {
                            let result = String::from_utf8(result)?;
                            return Ok(result);
                        }
                    }
                }
            }
        }
    }
    bail!("Should not exit while loop");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Manual test"]
    async fn test_mount() -> anyhow::Result<()> {
        let path = mount("sdc1").await?;
        println!("Path is {path}");
        Ok(())
    }

    #[tokio::test]
    #[ignore = "Manual test"]
    async fn test_unmount() -> anyhow::Result<()> {
        unmount("sdc1").await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore = "Manual test"]
    async fn test_wait_for_device() -> anyhow::Result<()> {
        let device = wait_for_device().await?;
        println!("Device {device}");

        Ok(())
    }
}
