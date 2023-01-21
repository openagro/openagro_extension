/// Copyright (c) 2023 OpenAgro Developers
///
/// This file under Lesser General Public License v3.0, please read accompanying file
/// copy or, read on https://www.gnu.org/licenses/lgpl-3.0.html

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AssetType {
    Javascript = 0,
    Css = 1,
    Views = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assets {
    pub asset_type: AssetType,
    pub assets: String,
}

