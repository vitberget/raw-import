use std::collections::HashMap;

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


#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    use anyhow::bail;

    #[tokio::test]
    async fn test_mount() -> anyhow::Result<()> {
        let path = mount("sdc1").await?;
        println!("Path is {path}");
        Ok(())
    }

    #[tokio::test]
    async fn test_unmount() -> anyhow::Result<()> {
        unmount("sdc1").await?;
        Ok(())
    }
}
