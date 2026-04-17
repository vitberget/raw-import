use std::collections::HashMap;

use anyhow::Context;
use futures_lite::StreamExt;

pub async fn mount(device: &str) -> anyhow::Result<String> {
    let mount = udisks2::Client::new().await?
        .object(device).context("No {device} device found")?
        .filesystem().await.context("No filesystem for {device}")?
        .mount(HashMap::new()).await?;
    Ok(mount)
}

pub async fn unmount(device: &str) -> anyhow::Result<()> {
    udisks2::Client::new().await?
        .object(device).context("No {device} device found")?
        .filesystem().await.context("No filesystem for {device}")?
        .unmount(HashMap::new()).await?;
    Ok(())
}

pub async fn get_partitions(device: &str) -> anyhow::Result<Vec<String>> {
    let partitions = udisks2::Client::new().await?
        .object(device).context("No {device} device found")?
        .partition_table().await.context("Failed to get partition_table for {device}")?
        .partitions().await.context("Failed to get partitions for {device}")?;

    Ok(partitions.iter().map(|owned| owned.to_string()).collect())
}

/// Returns something like "/org/freedesktop/UDisks2/block_devices/sda"
pub async fn wait_for_device() -> anyhow::Result<String> {
    let mut stream = udisks2::Client::new().await?
        .object_manager()
        .receive_interfaces_added().await?;

    // ASCII for "/org/freedesktop/UDisks2/block_devices/"
    const BLOCK_DEVICE: [u8; 39] = [
        47, 111, 114, 103, 47, 102, 114, 101, 101, 100, 101, 115, 107, 116, 111, 112, 47, 85, 68, 105, 115, 107, 115, 
        50, 47, 98, 108, 111, 99, 107, 95, 100, 101, 118, 105, 99, 101, 115, 47];

    loop { 
        if let Some(interface_added) = stream.next().await {
            let data = interface_added.message().data().bytes();
            let max = data.len() - BLOCK_DEVICE.len();

            for i in 0..=max {
                let delta_data = &data[i..];

                let same = delta_data.iter()
                    .zip(BLOCK_DEVICE)
                    .find(|(left, right)| *left != right)
                    .is_none();

                if same {
                    let result: Vec<u8> = delta_data.iter()
                        .take_while(|a_byte| **a_byte != 0)
                        .map(|a_byte| *a_byte)
                        .collect();

                    if let Some(character) = result.last() && !is_digit(character) {
                        return Ok(String::from_utf8(result)?);
                    }
                }
            }
        }
    }
}

fn is_digit(character: &u8) -> bool {
    const ZERO: u8 = 0x30;
    const NINE: u8 = 0x39;

    (ZERO..=NINE).contains(character)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_digit() {
        assert!(is_digit(&0x30));
        assert!(is_digit(&0x31));
        assert!(is_digit(&0x32));
        assert!(is_digit(&0x33));
        assert!(is_digit(&0x34));
        assert!(is_digit(&0x35));
        assert!(is_digit(&0x36));
        assert!(is_digit(&0x37));
        assert!(is_digit(&0x38));
        assert!(is_digit(&0x39));
    }

    #[test]
    fn test_is_not_digit() {
        assert!(!is_digit(&0x0));
        assert!(!is_digit(&0x2F));
        assert!(!is_digit(&0x40));
        assert!(!is_digit(&0x53));
        assert!(!is_digit(&0xFF));
    }

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

    #[tokio::test]
    #[ignore = "Manual test"]
    async fn test_partition() -> anyhow::Result<()> {
        let partitions = get_partitions("/org/freedesktop/UDisks2/block_devices/sda").await?;
        println!("Partitions {partitions:?}");
        Ok(())
    }
}
