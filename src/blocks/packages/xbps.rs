use std::env;
use std::process::Stdio;

use tokio::fs::{create_dir_all, File};
use tokio::process::Command;

use super::*;

#[derive(Default)]
pub struct Xbps;

impl Xbps {
    pub async fn new() -> Self {
        // let mut xbps = Xbps {};
        // xbps.setup().await?;

        Ok(Xbps {})
    }

    // async fn is_phased_update(&self, package_line: &str) -> Result<bool> {
    //     let package_name_regex = regex!(r#"(.*)/.*"#);
    //     let package_name = &package_name_regex
    //         .captures(package_line)
    //         .error("Couldn't find package name")?[1];
    //
    //     let output = String::from_utf8(
    //         Command::new("xbps-install")
    //             .args(["-c", &self.config_file, "policy", package_name])
    //             .output()
    //             .await
    //             .error("Problem running apt-cache command")?
    //             .stdout,
    //     )
    //     .error("Problem capturing apt-cache command output")?;
    //
    //     let phased_regex = regex!(r".*\(phased (\d+)%\).*");
    //     Ok(match phased_regex.captures(&output) {
    //         Some(matches) => &matches[1] != "100",
    //         None => false,
    //     })
    // }

    // async fn setup(&mut self) -> Result<()> {
    //     let mut cache_dir = env::temp_dir();
    //     cache_dir.push("i3rs-apt");
    //     if !cache_dir.exists() {
    //         create_dir_all(&cache_dir)
    //             .await
    //             .error("Failed to create temp dir")?;
    //     }
    //
    //     let apt_config = format!(
    //         "Dir::State \"{}\";\n
    //      Dir::State::lists \"lists\";\n
    //      Dir::Cache \"{}\";\n
    //      Dir::Cache::srcpkgcache \"srcpkgcache.bin\";\n
    //      Dir::Cache::pkgcache \"pkgcache.bin\";",
    //         cache_dir.display(),
    //         cache_dir.display(),
    //     );
    //
    //     let mut config_file = cache_dir;
    //     config_file.push("apt.conf");
    //     let config_file = config_file.to_str().unwrap();
    //
    //     self.config_file = config_file.to_string();
    //
    //     let mut file = File::create(&config_file)
    //         .await
    //         .error("Failed to create config file")?;
    //     file.write_all(apt_config.as_bytes())
    //         .await
    //         .error("Failed to write to config file")?;
    //
    //     Ok(())
    // }
}

#[async_trait]
impl Backend for Xbps {
    fn name(&self) -> Cow<'static, str> {
        "xbps".into()
    }

    async fn get_updates_list(&self) -> Result<Vec<String>> {
        let stdout = Command::new("xbps-install")
            .env("LC_LANG", "C")
            .args(["-M", "-u", "-n"])
            .output()
            .await
            .error("Problem running xbps-install command")?
            .stdout;

        let updates = String::from_utf8(stdout).expect("xbps-install produced non-UTF8 output");
        let updates_list: Vec<String> = updates
            .lines()
            .filter(|line| line.len() > 1)
            .map(|line| line.to_string())
            .collect();

        Ok(updates_list)
    }
}
