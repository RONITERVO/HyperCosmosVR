## Summary

Describe what changed and why.

## Audience Impact
- [ ] Desktop / VR user experience
- [ ] Developer / Shader / Audio architecture
- [ ] Content baking / Customization tooling
- [ ] Documentation / Legal / CI

## Verification Checklist
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes with zero warnings
- [ ] Unit tests pass cleanly (`cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey -p cosmos_xr` + `cosmos_spatial topology` + `cosmos_manifest` JSON/CRC32 tests)
- [ ] If graphics/rendering changed: ran local DX12 GPU tests + headless renders (`--headless-render`) and checked `#000000` OLED black & zero highlight blowout
- [ ] Confirmed zero proprietary `.ttf` fonts, zero `> 50 MiB` files, and zero machine-specific `.cargo/config.toml` tracked in Git

