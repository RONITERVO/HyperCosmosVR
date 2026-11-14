use std::fs;
use std::path::{Path, PathBuf};

use cosmos_manifest::{Locale, UniverseCatalog};
use cosmos_spatial::CosmosSpatialIndex;

use crate::odyssey::{CosmicHighlightsPlugin, GrandCosmicOdysseyPlugin};
use crate::schema::{JourneyError, StoryJourneyManifest};

/// Contract for modular code-based or data-backed Story-Journey plugins.
pub trait StoryJourneyPlugin: Send + Sync {
    fn id(&self) -> &str;
    fn title(&self, locale: Locale) -> &str;
    fn description(&self, locale: Locale) -> &str;
    fn is_builtin(&self) -> bool {
        true
    }
    fn build_manifest(
        &self,
        catalog: &UniverseCatalog,
        spatial: &CosmosSpatialIndex,
    ) -> Result<StoryJourneyManifest, JourneyError>;
}

/// Plugin adapter backed by an external portable `.hcj.json` manifest file or share URI.
pub struct JsonFileJourneyPlugin {
    manifest: StoryJourneyManifest,
    source_path: Option<PathBuf>,
}

impl JsonFileJourneyPlugin {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, JourneyError> {
        let path_ref = path.as_ref();
        let text = fs::read_to_string(path_ref)?;
        let manifest = StoryJourneyManifest::from_json_str(&text)?;
        Ok(Self {
            manifest,
            source_path: Some(path_ref.to_path_buf()),
        })
    }

    pub fn from_share_uri(uri: &str) -> Result<Self, JourneyError> {
        let manifest = StoryJourneyManifest::decode_share_uri(uri)?;
        Ok(Self {
            manifest,
            source_path: None,
        })
    }

    pub fn source_path(&self) -> Option<&Path> {
        self.source_path.as_deref()
    }
}

impl StoryJourneyPlugin for JsonFileJourneyPlugin {
    fn id(&self) -> &str {
        &self.manifest.id
    }

    fn title(&self, locale: Locale) -> &str {
        self.manifest.title.pick(locale)
    }

    fn description(&self, locale: Locale) -> &str {
        self.manifest.subtitle.pick(locale)
    }

    fn is_builtin(&self) -> bool {
        false
    }

    fn build_manifest(
        &self,
        catalog: &UniverseCatalog,
        _spatial: &CosmosSpatialIndex,
    ) -> Result<StoryJourneyManifest, JourneyError> {
        self.manifest.validate(Some(catalog))?;
        Ok(self.manifest.clone())
    }
}

/// Lightweight UI/CLI summary describing one registered Story-Journey plugin.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JourneySummary {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_builtin: bool,
    pub source_path: Option<PathBuf>,
}

/// Central registry managing built-in Rust journey plugins, `.hcj.json` files, and share links.
pub struct JourneyRegistry {
    entries: Vec<(Box<dyn StoryJourneyPlugin>, Option<PathBuf>)>,
}

impl Default for JourneyRegistry {
    fn default() -> Self {
        Self::with_builtins()
    }
}

impl JourneyRegistry {
    /// Creates a registry pre-populated with `GrandCosmicOdysseyPlugin` (default) and `CosmicHighlightsPlugin`.
    pub fn with_builtins() -> Self {
        let mut reg = Self {
            entries: Vec::new(),
        };
        reg.register_plugin(Box::new(GrandCosmicOdysseyPlugin), None);
        reg.register_plugin(Box::new(CosmicHighlightsPlugin), None);
        reg
    }

    pub fn register_plugin(
        &mut self,
        plugin: Box<dyn StoryJourneyPlugin>,
        source_path: Option<PathBuf>,
    ) {
        let id = plugin.id().to_owned();
        if let Some(slot) = self.entries.iter_mut().find(|(p, _)| p.id() == id) {
            *slot = (plugin, source_path);
        } else {
            self.entries.push((plugin, source_path));
        }
    }

    pub fn register_file(&mut self, path: impl AsRef<Path>) -> Result<String, JourneyError> {
        let path_buf = path.as_ref().to_path_buf();
        let plugin = JsonFileJourneyPlugin::from_file(&path_buf)?;
        let id = plugin.id().to_owned();
        self.register_plugin(Box::new(plugin), Some(path_buf));
        Ok(id)
    }

    pub fn register_share_uri(&mut self, uri: &str) -> Result<String, JourneyError> {
        let plugin = JsonFileJourneyPlugin::from_share_uri(uri)?;
        let id = plugin.id().to_owned();
        self.register_plugin(Box::new(plugin), None);
        Ok(id)
    }

    /// Scans a folder for `*.hcj.json` files (non-fatal if directory does not exist yet).
    pub fn discover_in_directory(&mut self, dir: impl AsRef<Path>) -> Result<usize, JourneyError> {
        let dir = dir.as_ref();
        if !dir.is_dir() {
            return Ok(0);
        }
        let mut files: Vec<PathBuf> = fs::read_dir(dir)?
            .filter_map(|e| e.ok().map(|de| de.path()))
            .filter(|p| {
                p.is_file()
                    && p.file_name()
                        .and_then(|s| s.to_str())
                        .is_some_and(|n| n.ends_with(".hcj.json"))
            })
            .collect();
        files.sort();
        let mut count = 0usize;
        for file in files {
            self.register_file(&file)?;
            count += 1;
        }
        Ok(count)
    }

    pub fn summaries(&self, locale: Locale) -> Vec<JourneySummary> {
        self.entries
            .iter()
            .map(|(p, path)| JourneySummary {
                id: p.id().to_owned(),
                title: p.title(locale).to_owned(),
                description: p.description(locale).to_owned(),
                is_builtin: p.is_builtin(),
                source_path: path.clone(),
            })
            .collect()
    }

    pub fn default_id(&self) -> &str {
        self.entries
            .first()
            .map(|(p, _)| p.id())
            .unwrap_or("grand_cosmic_odyssey")
    }

    /// Resolves a journey by plugin id, filesystem path (`*.hcj.json`), or `hypercosmos://journey?hcj=...` URI.
    pub fn resolve(
        &mut self,
        id_or_path_or_uri: &str,
        catalog: &UniverseCatalog,
        spatial: &CosmosSpatialIndex,
    ) -> Result<StoryJourneyManifest, JourneyError> {
        let query = id_or_path_or_uri.trim();
        if query.starts_with(crate::schema::SHARE_URI_PREFIX) {
            let id = self.register_share_uri(query)?;
            return self.resolve_by_id(&id, catalog, spatial);
        }
        let path = Path::new(query);
        if path.is_file() {
            let id = self.register_file(path)?;
            return self.resolve_by_id(&id, catalog, spatial);
        }
        self.resolve_by_id(query, catalog, spatial)
    }

    fn resolve_by_id(
        &self,
        id: &str,
        catalog: &UniverseCatalog,
        spatial: &CosmosSpatialIndex,
    ) -> Result<StoryJourneyManifest, JourneyError> {
        if let Some((plugin, _)) = self.entries.iter().find(|(p, _)| p.id() == id) {
            return plugin.build_manifest(catalog, spatial);
        }
        Err(JourneyError::JourneyNotFound(id.to_owned()))
    }
}
