use std::collections::HashMap;

use cosmos_manifest::Locale;
use serde::{Deserialize, Serialize};

/// Segment roles eligible for deduplication/skipping by [`SharedNarrationMemory`].
///
/// Matches the exact `Spanish-Quick-Apps` contract: ONLY `region-heading` and `shared-context`
/// participate; item names and item explanations always repeat cleanly.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum MemoryRole {
    RegionHeading,
    SharedContext,
}

impl MemoryRole {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::RegionHeading => "region-heading",
            Self::SharedContext => "shared-context",
        }
    }
}

/// Read-only view of one narration segment evaluated by [`SharedNarrationMemory::begin_plan`].
#[derive(Clone, Debug)]
pub struct MemorySegmentInput<'a> {
    pub id: &'a str,
    pub role: Option<MemoryRole>,
    pub display: &'a str,
    pub narration: &'a str,
    pub translation: &'a str,
}

/// Remembered entry stored per `(region_id, segment_id)`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RememberedEntry {
    pub role: MemoryRole,
    pub text_hash: u64,
}

/// Ephemeral token/plan computed when a narration run begins.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryPlanState {
    pub generation: u64,
    pub region_id: String,
    /// Parallel to input segments: `true` if already heard completely in this region and should be skipped.
    pub skipped: Vec<bool>,
    /// Normalized key for each eligible non-duplicate segment (`None` if ineligible or duplicate ID).
    pub eligible_entries: Vec<Option<(String, RememberedEntry)>>,
}

/// Deterministic 64-bit FNV-1a hash over `(role, display, narration, locale, translation)` matching
/// `Spanish-Quick-Apps/src/shared/shared-narration-memory.js` normalization semantics.
#[inline]
pub fn compute_segment_text_hash(
    role: MemoryRole,
    display: &str,
    narration: &str,
    locale: Locale,
    translation: &str,
) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0100_0000_01b3;

    let mut h = FNV_OFFSET;
    let mut feed = |bytes: &[u8]| {
        for &b in bytes {
            h ^= b as u64;
            h = h.wrapping_mul(FNV_PRIME);
        }
        // Null delimiter prevents cross-field concatenation collisions.
        h ^= 0xff;
        h = h.wrapping_mul(FNV_PRIME);
    };

    feed(role.as_str().as_bytes());
    feed(display.trim().as_bytes());
    feed(narration.trim().as_bytes());
    feed(locale.code().as_bytes());
    feed(translation.trim().as_bytes());
    h
}

/// Scoped bilingual shared-context deduplication matching `createSharedNarrationMemory`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SharedNarrationMemory {
    current_region: Option<String>,
    generation: u64,
    heard: HashMap<String, RememberedEntry>,
}

impl SharedNarrationMemory {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn current_region(&self) -> Option<&str> {
        self.current_region.as_deref()
    }

    #[inline]
    pub fn generation(&self) -> u64 {
        self.generation
    }

    #[inline]
    pub fn remembered_count(&self) -> usize {
        self.heard.len()
    }

    /// Returns the stored entry for `segment_id` in the active region, if any.
    #[inline]
    pub fn remembered(&self, segment_id: &str) -> Option<RememberedEntry> {
        self.heard.get(segment_id.trim()).copied()
    }

    /// Clears all remembered segments and bumps generation so in-flight callbacks become stale.
    pub fn reset(&mut self) {
        self.current_region = None;
        self.heard.clear();
        self.generation = self.generation.wrapping_add(1);
    }

    /// Invalidates any in-flight narration run (e.g. on movement cancel) without forgetting completed context.
    pub fn invalidate_pending(&mut self) {
        self.generation = self.generation.wrapping_add(1);
    }

    /// Switches active region; clears remembered entries immediately if `region_id` changed.
    pub fn set_region(&mut self, region_id: Option<&str>) {
        let next = region_id
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned);
        if self.current_region != next {
            self.current_region = next;
            self.heard.clear();
            self.generation = self.generation.wrapping_add(1);
        }
    }

    /// Evaluates which segments in a narration plan can be skipped because their region-heading
    /// or shared-context pair already finished completely (Spanish + Translation) in `region_id`.
    pub fn begin_plan(
        &mut self,
        region_id: &str,
        segments: &[MemorySegmentInput<'_>],
        locale: Locale,
    ) -> MemoryPlanState {
        let norm_region = region_id.trim();
        if norm_region.is_empty() {
            self.reset();
            return MemoryPlanState {
                generation: self.generation,
                region_id: String::new(),
                skipped: vec![false; segments.len()],
                eligible_entries: vec![None; segments.len()],
            };
        }

        self.set_region(Some(norm_region));

        let mut id_counts: HashMap<&str, usize> = HashMap::new();
        let mut raw_keys: Vec<Option<(&str, RememberedEntry)>> = Vec::with_capacity(segments.len());

        for seg in segments {
            let id = seg.id.trim();
            let display = seg.display.trim();
            let narration = seg.narration.trim();
            let translation = seg.translation.trim();

            if let Some(role) = seg.role
                && !id.is_empty()
                && !display.is_empty()
                && !narration.is_empty()
                && !translation.is_empty()
            {
                *id_counts.entry(id).or_insert(0) += 1;
                let text_hash =
                    compute_segment_text_hash(role, display, narration, locale, translation);
                raw_keys.push(Some((id, RememberedEntry { role, text_hash })));
                continue;
            }
            raw_keys.push(None);
        }

        let mut skipped = Vec::with_capacity(segments.len());
        let mut eligible_entries = Vec::with_capacity(segments.len());

        for raw in raw_keys {
            if let Some((id, entry)) = raw
                && id_counts.get(id).copied() == Some(1)
            {
                if let Some(prev) = self.heard.get(id) {
                    if *prev == entry {
                        skipped.push(true);
                        eligible_entries.push(Some((id.to_owned(), entry)));
                        continue;
                    }
                    // Same region & ID, different text/role/locale: purge stale cache entry.
                    self.heard.remove(id);
                }
                skipped.push(false);
                eligible_entries.push(Some((id.to_owned(), entry)));
                continue;
            }
            skipped.push(false);
            eligible_entries.push(None);
        }

        MemoryPlanState {
            generation: self.generation,
            region_id: norm_region.to_owned(),
            skipped,
            eligible_entries,
        }
    }

    /// Marks `segment_index` as remembered ONLY when BOTH Spanish and Translation finished uninterrupted
    /// within the same generation and region.
    pub fn mark_pair_completed(
        &mut self,
        state: &MemoryPlanState,
        segment_index: usize,
        spanish_done: bool,
        translation_done: bool,
    ) -> bool {
        if !(spanish_done && translation_done) {
            return false;
        }
        if state.generation != self.generation {
            return false;
        }
        if self.current_region.as_deref() != Some(state.region_id.as_str()) {
            return false;
        }
        let Some(Some((id, entry))) = state.eligible_entries.get(segment_index) else {
            return false;
        };
        self.heard.insert(id.clone(), *entry);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_segments() -> Vec<MemorySegmentInput<'static>> {
        vec![
            MemorySegmentInput {
                id: "evidence-class",
                role: Some(MemoryRole::SharedContext),
                display: "Consenso establecido",
                narration: "Consenso establecido",
                translation: "Established consensus",
            },
            MemorySegmentInput {
                id: "zone-name",
                role: Some(MemoryRole::RegionHeading),
                display: "Mesosfera",
                narration: "Mesosfera",
                translation: "Mesosphere",
            },
            MemorySegmentInput {
                id: "feature-name",
                role: None, // `Item` must never be remembered.
                display: "Nubes noctilucentes",
                narration: "Nubes noctilucentes",
                translation: "Noctilucent clouds",
            },
            MemorySegmentInput {
                id: "metric",
                role: Some(MemoryRole::SharedContext),
                display: "50-85 km",
                narration: "cincuenta a ochenta y cinco kilómetros",
                translation: "50 to 85 kilometers",
            },
            MemorySegmentInput {
                id: "fact",
                role: None, // `Explanation` must never be remembered.
                display: "Cristales de hielo a gran altura.",
                narration: "Cristales de hielo a gran altura.",
                translation: "High-altitude ice crystals.",
            },
        ]
    }

    #[test]
    fn remembers_only_shared_roles_after_both_es_and_translation_finish() {
        let mut mem = SharedNarrationMemory::new();
        let segs = sample_segments();
        let state = mem.begin_plan("03:mesosfera", &segs, Locale::EnUs);
        assert_eq!(state.skipped, vec![false, false, false, false, false]);

        // Interrupted before translation completes -> MUST NOT remember.
        assert!(!mem.mark_pair_completed(&state, 0, true, false));
        assert!(!mem.mark_pair_completed(&state, 1, false, true));
        assert_eq!(mem.remembered_count(), 0);

        // Both Spanish & Translation completed uninterrupted:
        assert!(mem.mark_pair_completed(&state, 0, true, true));
        assert!(mem.mark_pair_completed(&state, 1, true, true));
        // Item & Explanation are ineligible even when completed:
        assert!(!mem.mark_pair_completed(&state, 2, true, true));
        assert!(mem.mark_pair_completed(&state, 3, true, true));
        assert!(!mem.mark_pair_completed(&state, 4, true, true));
        assert_eq!(mem.remembered_count(), 3);

        // Revisiting another feature in the same region skips heading/context, keeps item/fact!
        let next_state = mem.begin_plan("03:mesosfera", &segs, Locale::EnUs);
        assert_eq!(next_state.skipped, vec![true, true, false, true, false]);
    }

    #[test]
    fn region_change_and_pending_invalidation_clear_or_reject() {
        let mut mem = SharedNarrationMemory::new();
        let segs = sample_segments();
        let s1 = mem.begin_plan("03:mesosfera", &segs, Locale::EnUs);
        assert!(mem.mark_pair_completed(&s1, 1, true, true));
        assert_eq!(mem.remembered_count(), 1);

        // Interrupting narration increments generation so stale callback cannot commit.
        let s2 = mem.begin_plan("03:mesosfera", &segs, Locale::EnUs);
        mem.invalidate_pending();
        assert!(!mem.mark_pair_completed(&s2, 3, true, true));

        // Region change wipes remembered cache immediately!
        let s3 = mem.begin_plan("03:estratosfera", &segs, Locale::EnUs);
        assert_eq!(mem.remembered_count(), 0);
        assert_eq!(s3.skipped, vec![false; 5]);
        assert!(!mem.mark_pair_completed(&s1, 3, true, true));
    }

    #[test]
    fn locale_or_text_change_and_duplicate_id_do_not_skip() {
        let mut mem = SharedNarrationMemory::new();
        let segs = sample_segments();
        let s1 = mem.begin_plan("04:sol", &segs, Locale::EnUs);
        assert!(mem.mark_pair_completed(&s1, 1, true, true));

        // Switching EN -> FI changes hash, purging stale entry instead of skipping.
        let s_fi = mem.begin_plan("04:sol", &segs, Locale::FiFi);
        assert!(!s_fi.skipped[1]);
        assert!(mem.remembered("zone-name").is_none());

        // Duplicate segment ID within same plan disqualifies both from shared memory.
        let dups = vec![
            MemorySegmentInput {
                id: "metric",
                role: Some(MemoryRole::SharedContext),
                display: "A",
                narration: "A",
                translation: "A",
            },
            MemorySegmentInput {
                id: "metric",
                role: Some(MemoryRole::SharedContext),
                display: "B",
                narration: "B",
                translation: "B",
            },
        ];
        let s_dup = mem.begin_plan("04:sol", &dups, Locale::EnUs);
        assert_eq!(s_dup.skipped, vec![false, false]);
        assert!(!mem.mark_pair_completed(&s_dup, 0, true, true));
    }
}
