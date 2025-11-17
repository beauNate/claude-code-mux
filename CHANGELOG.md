# Changelog

All notable changes to Claude Code Mux will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.3] - 2025-11-17

### Added
- CI and Latest Release badges to README
- FAQ section with 6 common questions
- CHANGELOG.md with full version history
- Collapsible screenshots with descriptive captions
- Collapsible provider details section

### Changed
- Restructured README for better onboarding flow (moved comparison section to bottom)
- Compressed Supported Providers section with summary
- Updated performance metrics with actual measurements (6MB vs 156MB)
- Improved OAuth description to focus on Claude Pro/Max compatibility

### Fixed
- Memory usage comparison (updated from 10x to accurate 25x difference)

## [0.4.2] - 2025-11-17

### Fixed
- Use rustls instead of native-tls for better cross-compilation support

### Changed
- Added automated release workflow for GitHub releases

## [0.4.1] - 2025-11-17

### Fixed
- Use `/v1/responses` endpoint for Codex model streaming requests

## [0.4.0] - 2025-11-17

### Added
- OpenAI Responses API support for Codex models (gpt-5-turbo, etc.)
- Automatic endpoint routing based on model type

## [0.3.0] - 2025-11-17

### Added
- OpenAI-compatible `/v1/chat/completions` endpoint
- Support for OpenAI format requests alongside Anthropic format

### Fixed
- Router tab auto-save logging improvements

## [0.2.0] - 2025-11-17

### Added
- Documentation improvements
- Engaging intro tagline in README

## [0.1.0] - 2025-11-17

### Added
- Initial release of Claude Code Mux
- High-performance AI routing proxy built in Rust
- Anthropic Messages API compatibility (`/v1/messages`)
- Intelligent model routing (default, think, background, websearch)
- Provider failover with priority-based routing
- Streaming support (SSE)
- Web-based admin UI with auto-save
- OAuth 2.0 authentication for Anthropic
- Multi-provider support (16+ providers)
- Auto-mapping with regex patterns
- TOML-based configuration
- Token counting endpoint (`/v1/messages/count_tokens`)

[0.4.3]: https://github.com/9j/claude-code-mux/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/9j/claude-code-mux/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/9j/claude-code-mux/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/9j/claude-code-mux/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/9j/claude-code-mux/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/9j/claude-code-mux/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/9j/claude-code-mux/releases/tag/v0.1.0
