// Copyright 2022. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

use std::path::{Path, PathBuf};

use anyhow::Error;
use tokio::fs;

macro_rules! embed_file {
    ($f:literal) => {
        ConfigFile::new($f, include_str!(concat!("../../assets/", $f)))
    };
}

const CONFIG_TOML: ConfigFile = embed_file!("config.toml");
const DEFAULTS_INI: ConfigFile = embed_file!("defaults.ini");
const LOGS4RS_YML: ConfigFile = embed_file!("log4rs.yml");
const LOKI_YML: ConfigFile = embed_file!("loki_config.yml");
const PROMTAIL_YML: ConfigFile = embed_file!("promtail.config.yml");
const PROVISION_YML: ConfigFile = embed_file!("sources_provision.yml");
const LOG4RS_CLI_YML: ConfigFile = embed_file!("log4rs-cli.yml");

struct ConfigFile {
    filename: &'static str,
    data: &'static str,
}

impl ConfigFile {
    const fn new(filename: &'static str, data: &'static str) -> Self {
        Self { filename, data }
    }
}

pub struct Configurator {
    base_dir: PathBuf,
}

impl Configurator {
    pub fn init() -> Result<Self, Error> {
        let cache_dir = dirs_next::cache_dir().ok_or_else(|| Error::msg("No cache dir"))?;
        let mut data_directory = cache_dir;
        data_directory.push("tari-launchpad");
        Ok(Self {
            base_dir: data_directory,
        })
    }

    pub fn base_path(&self) -> &PathBuf {
        &self.base_dir
    }

    // pub async fn read_config(&self) -> Result<LaunchpadConfig, Error> {
    // let mut path = self.base_dir.clone();
    // path.push("config");
    // path.push("config.toml");
    // let data = fs::read_to_string(&path).await?;
    // let config = toml::from_str(&data)?;
    // Ok(config)
    // }

    async fn create_dir(&mut self, folder: &Path) -> Result<(), Error> {
        if !folder.exists() {
            fs::create_dir_all(&folder).await?;
        }
        Ok(())
    }

    async fn create_sub_dir(&mut self, folder: &Path, sub_path: &str) -> Result<PathBuf, Error> {
        let mut path = folder.to_path_buf();
        path.push(sub_path);
        if !path.exists() {
            fs::create_dir_all(&path).await?;
        }
        Ok(path)
    }

    async fn store_file(&mut self, folder: &Path, file: &ConfigFile) -> Result<(), Error> {
        let mut path = folder.to_path_buf();
        path.push(file.filename);
        if !path.exists() {
            fs::write(path, file.data).await?;
        }
        Ok(())
    }

    pub async fn clean_configuration(&mut self) -> Result<(), Error> {
        let base_dir = self.base_dir.clone();
        let config_dir = self.create_sub_dir(&base_dir, "config").await?;
        tokio::fs::remove_dir_all(config_dir).await?;
        Ok(())
    }

    pub async fn init_configuration(&mut self) -> Result<(), Error> {
        // base path
        let base_dir = self.base_dir.clone();
        self.create_dir(&base_dir).await?;
        let config_dir = self.create_sub_dir(&base_dir, "config").await?;
        // config files
        self.store_file(&config_dir, &CONFIG_TOML).await?;
        self.store_file(&config_dir, &DEFAULTS_INI).await?;
        self.store_file(&config_dir, &LOGS4RS_YML).await?;
        self.store_file(&config_dir, &LOKI_YML).await?;
        self.store_file(&config_dir, &PROMTAIL_YML).await?;
        self.store_file(&config_dir, &PROVISION_YML).await?;

        self.create_sub_dir(&base_dir, "log").await?;
        self.store_file(&config_dir, &LOG4RS_CLI_YML).await?;

        // TODO: Use `enum` here...
        // images
        self.create_sub_dir(&base_dir, "tor").await?;
        self.create_sub_dir(&base_dir, "base_node").await?;
        self.create_sub_dir(&base_dir, "wallet").await?;
        self.create_sub_dir(&base_dir, "xmrig").await?;
        self.create_sub_dir(&base_dir, "sha3_miner").await?;
        self.create_sub_dir(&base_dir, "mm_proxy").await?;
        self.create_sub_dir(&base_dir, "monerod").await?;
        self.create_sub_dir(&base_dir, "grafana").await?;
        Ok(())
    }
}
