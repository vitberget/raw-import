use std::collections::HashMap;

use anyhow::{Context, bail};
use futures_lite::StreamExt;
use log::debug;
use udisks2::zbus::OwnedObjectPath;

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

pub async fn print_udisk_status() -> anyhow::Result<()> {
    let client = udisks2::Client::new().await?;
    debug!("Current drives:");
    for object in client
        .object_manager()
        .get_managed_objects()
        .await?.into_keys()
        .filter_map(|object_path| client.object(object_path).ok())
    {
        let Ok(drive) = object.drive().await else { continue; };

        debug!( "  {model}: {size}",
            model = drive.model().await?,
            size = client.size_for_display(drive.size().await?, false, false)
        );
    }

    Ok(())
}

/// Returns something like "/org/freedesktop/UDisks2/block_devices/sda"
pub async fn wait_for_card() -> anyhow::Result<String> {
    let mut stream = udisks2::Client::new().await?
        .object_manager()
        .receive_interfaces_added().await?;

    const BLOCK_DEVICE: [u8; 39] = [
        // ASCII for "/org/freedesktop/UDisks2/block_devices/"
        b'/', b'o', b'r', b'g', b'/', b'f', b'r', b'e', b'e', b'd', b'e', b's', b'k', b't', b'o', b'p', b'/', 
        b'U', b'D', b'i', b's', b'k', b's', b'2', b'/', b'b', b'l', b'o', b'c', b'k', b'_', b'd', b'e', b'v', 
        b'i', b'c', b'e', b's', b'/'
    ];

    loop { 
        if let Some(interface_added) = stream.next().await {
            let data = interface_added.message().data().bytes();
            let max = data.len() - BLOCK_DEVICE.len();

            for i in 0..=max {
                let delta_data = &data[i..];

                let matches_block_device_text = delta_data.iter()
                    .zip(BLOCK_DEVICE)
                    .find(|(left, right)| *left != right)
                    .is_none();

                if matches_block_device_text {
                    let result: Vec<u8> = delta_data.iter()
                        .take_while(|a_byte| **a_byte != 0)
                        .copied()
                        .collect();

                    if let Some(character) = result.last() && !is_digit(character) {
                        return Ok(String::from_utf8(result)?);
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub info: String,
    pub partitions: Vec<String>
}

pub async fn get_cards() -> anyhow::Result<Vec<Card>> {
    let mut cards: Vec<Card> = vec![];

    let client = udisks2::Client::new().await?;

    if let Ok(drives) = client.manager().get_drives(HashMap::new()).await {
        for drive_path in drives {
            if is_removable(&drive_path).await && let Ok(partitions) = get_partitions_for_thing(&drive_path).await {
                let info = get_drive_info(&drive_path).await?;
                cards.push(Card { info, partitions })
            }
        }
    }

    Ok(cards)
}

async fn is_removable(drive_path: &OwnedObjectPath) -> bool {
    if let Ok(client) = udisks2::Client::new().await 
    && let Ok(object) = client.object(drive_path.to_owned()) 
    && let Ok(drive) = object.drive().await
    {
        return drive.removable().await.unwrap_or(false) && drive.size().await.unwrap_or(0) > 0;
    }

    false
}

async fn get_drive_info(drive_path: &OwnedObjectPath) -> anyhow::Result<String> {
    let client = udisks2::Client::new().await?;

    let object = client.object(drive_path.to_owned())?;
    let info = client.object_info(&object).await;

    Ok(format!("{name} {description}", 
        name = info.name.unwrap_or("NoName".to_string()), 
        description = info.description.unwrap_or("NoDescription".to_string())
    ).to_string())
}

async fn get_partitions_for_thing(drive_path: &OwnedObjectPath) -> anyhow::Result<Vec<String>> {
    let client = udisks2::Client::new().await?;

    for block_device_path in client.manager().get_block_devices(HashMap::new()).await? {
        let object = client.object(block_device_path)?;
        if let Ok(block) = object.block().await
        && Ok(drive_path) == block.drive().await.as_ref()
        && let Ok(partition_table) = object.partition_table().await
        && let Ok(partitions) = partition_table.partitions().await {
            return Ok(partitions.iter().map(|p| p.to_string()).collect())
        }
    }

    bail!("No partitions")
}

pub async fn get_partitions(device: &str) -> anyhow::Result<Vec<String>> {
    let partitions = udisks2::Client::new().await?
        .object(device).context("No {device} device found")?
        .partition_table().await.context("Failed to get partition_table for {device}")?
        .partitions().await.context("Failed to get partitions for {device}")?;

    Ok(partitions.iter().map(|p| p.to_string()).collect())
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
        let path = mount("/org/freedesktop/UDisks2/block_devices/sda1").await?;
        println!("Path is {path}");
        Ok(())
    }

    #[tokio::test]
    #[ignore = "Manual test"]
    async fn test_unmount() -> anyhow::Result<()> {
        unmount("/org/freedesktop/UDisks2/block_devices/sda1").await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore = "Manual test"]
    async fn test_wait_for_device() -> anyhow::Result<()> {
        let device = wait_for_card().await?;
        println!("Device {device}");

        Ok(())
    }

    #[tokio::test]
    #[ignore = "Manual test"]
    async fn test_partition() -> anyhow::Result<()> {
        let partitions = get_partitions("/org/freedesktop/UDisks2/block_devices/sdc").await?;
        println!("Partitions {partitions:?}");
        Ok(())
    }
}
