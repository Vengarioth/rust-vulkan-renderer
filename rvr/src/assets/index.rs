use crate::Error;
use hashbrown::HashMap;
use rvr_assets::AssetType;

pub fn load_index(path: &str) -> Result<AssetIndex, Error> {
    use bincode;

    let contents = std::fs::read(path)?;
    let result = bincode::deserialize::<rvr_assets::AssetIndex>(&contents)?;

    let mut assets = HashMap::new();

    for location in result.locations {
        assets.insert(location.name, AssetLocation {
            asset_type: location.asset_type,
            path: format!("./dist/{}", result.files[location.file_index]),
            offset: location.offset,
            length: location.length,
        });
    }

    Ok(AssetIndex::new(assets))
}

#[derive(Debug)]
pub struct AssetLocation {
    pub asset_type: AssetType,
    pub path: String,
    pub offset: usize,
    pub length: usize,
}

#[derive(Debug)]
pub struct AssetIndex {
    assets: HashMap<String, AssetLocation>,
}

impl AssetIndex {
    pub fn new(assets: HashMap<String, AssetLocation>) -> Self {
        Self {
            assets,
        }
    }

    pub fn find_location(&self, name: &str) -> Option<&AssetLocation> {
        self.assets.get(name)
    }
}
