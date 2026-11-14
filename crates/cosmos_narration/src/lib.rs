pub mod fsm;
pub mod memory;
pub mod resolver;

pub use fsm::{
    FocusCandidate, FsmConfig, FsmInput, FsmOutput, FsmState, FsmStateKind, RestNarrationFsm,
    TargetIdentity,
};
pub use memory::{
    compute_segment_text_hash, MemoryPlanState, MemoryRole, MemorySegmentInput, RememberedEntry,
    SharedNarrationMemory,
};
pub use resolver::{
    encode_cue_tags_mask, hsl_to_srgb_u8, srgb_to_linear_hdr, AudioAssetIds,
    ChromaticVisualHint, DocumentaryCueSnapshot, EvidencePolicy, HarmonicVisualHint,
    MusicTriggerEvent, NarrationPlan, NarrationSegment, PhraseTripleIds, ResolverConfig,
    ResolverError, SegmentRole, SegmentVisualHint, TargetFocus, TargetResolver, VisualCueTag,
};
