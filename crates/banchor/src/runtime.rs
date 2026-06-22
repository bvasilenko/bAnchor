use banchor::corpus_index::TaskClassCorpusIndex;
use bsuite_core::{
    BinaryDefaults, BsuiteCoreError, FileSystemManifestOverlayReader, FileSystemTranscriptAppender,
    FullAdapterHostBinder, HostContext, HostInvocationContext, ManifestOverlay,
    ManifestOverlayReader,
};
use ed25519_dalek::VerifyingKey;
use std::path::{Path, PathBuf};

const CORPUS_TOML: &str = include_str!("../corpus/banchor-v0.toml");
const PUBKEY_BYTES: &[u8] = include_bytes!("../corpus/banchor-v0-pubkey.bin");

pub const EMBEDDED_CORPUS_VERSION: u32 = 1;

pub struct BinaryRuntime {
    pub corpus: TaskClassCorpusIndex,
    pub appender: FileSystemTranscriptAppender,
    pub host_context: HostContext,
    pub invocation_context: Option<HostInvocationContext>,
    pub install_dir: PathBuf,
    pub corpus_version: u32,
}

impl BinaryRuntime {
    pub fn init(install_dir: PathBuf) -> Result<Self, BsuiteCoreError> {
        let corpus = load_corpus()?;
        let defaults = load_defaults(&install_dir)?;
        let appender = FileSystemTranscriptAppender::from_base_dir(
            defaults.transcript_dir,
            defaults.transcript_retention_days,
        );
        let host_binder = FullAdapterHostBinder::from_env()?;
        Ok(Self {
            corpus,
            appender,
            host_context: host_binder.resolved_host_context(),
            invocation_context: host_binder.invocation_context().cloned(),
            install_dir,
            corpus_version: EMBEDDED_CORPUS_VERSION,
        })
    }
}

fn load_corpus() -> Result<TaskClassCorpusIndex, BsuiteCoreError> {
    let pubkey = load_pubkey()?;
    TaskClassCorpusIndex::from_toml_signed(CORPUS_TOML, &pubkey)
        .map_err(|e| BsuiteCoreError::CorpusDeserializationFailed(e.to_string()))
}

fn load_pubkey() -> Result<VerifyingKey, BsuiteCoreError> {
    let bytes: [u8; 32] = PUBKEY_BYTES.try_into().map_err(|_| {
        BsuiteCoreError::CorpusDeserializationFailed("embedded pubkey is not 32 bytes".to_owned())
    })?;
    VerifyingKey::from_bytes(&bytes)
        .map_err(|e| BsuiteCoreError::CorpusDeserializationFailed(e.to_string()))
}

fn load_defaults(install_dir: &Path) -> Result<BinaryDefaults, BsuiteCoreError> {
    let base_dir = FileSystemTranscriptAppender::new("banchor")?
        .directory()
        .to_path_buf();
    let overlay_reader = FileSystemManifestOverlayReader::new("banchor", install_dir);
    let overlay = overlay_reader
        .read()
        .unwrap_or_else(|_| ManifestOverlay::empty());
    let mut defaults = BinaryDefaults {
        transcript_retention_days: 90,
        transcript_dir: base_dir,
        corpus_dir: install_dir.to_path_buf(),
        update_check_interval_minutes: 60,
        stdout_byte_cap: 65536,
        binary_timeout_ms: 5000,
    };
    overlay.merge_into_defaults(&mut defaults);
    Ok(defaults)
}
