# Changelog

## [0.4.2](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/compare/v0.4.1...v0.4.2) (2026-03-13)


### Features

* send (de facto) empty message on invalid JSON input ([#17](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/issues/17)) ([fb8e950](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/commit/fb8e95047cc912648745ae293e2e4b03fc58864c))

## [0.4.1](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/compare/v0.4.0...v0.4.1) (2026-03-02)


### Features

* add graceful shutdown ([cfb3216](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/commit/cfb3216b089c46688416040f8c9516f2c3a82ebf))
* re-use request id from HTTP header ([#15](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/issues/15)) ([291413b](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/commit/291413b7c5683e6941de8d563ad442311c2638ea))

## [0.4.0](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/compare/v0.3.0...v0.4.0) (2026-01-07)


### ⚠ BREAKING CHANGES

* add the request method to record header ([#12](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/issues/12))

### Features

* add DELETE endpoint as used in DNPM:DIP ([#10](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/issues/10)) ([b46f392](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/commit/b46f392c37b99edea1945bb6181b23fce8424cfe))
* add the request method to record header ([#12](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/issues/12)) ([9f72c34](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/commit/9f72c34cc726e4035dfe3c47ad3e33f919e3905e))

## [0.3.0](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/compare/v0.2.0...v0.3.0) (2025-12-15)


### ⚠ BREAKING CHANGES

* update dto lib to version 0.2.0 ([#8](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/issues/8))

### deps

* update dto lib to version 0.2.0 ([#8](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/issues/8)) ([c71d707](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/commit/c71d707c3c22c83c8d859e380b2610ee51677b88))

## [0.2.0](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/compare/v0.1.2...v0.2.0) (2025-11-21)


### ⚠ BREAKING CHANGES

* downgrade of mtb-dto due to delayed update of DNPM:DIP ([#6](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/issues/6))

### Bug Fixes

* downgrade of mtb-dto due to delayed update of DNPM:DIP ([#6](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/issues/6)) ([d94a7e0](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/commit/d94a7e00fbd9cf1463fc5abe773a7886facb93ee))

## [0.1.2](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/compare/v0.1.1...v0.1.2) (2025-11-20)


### Bug Fixes

* add required import for release build ([0a05864](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/commit/0a05864051f1745ae8b81f67cffd5a25778755d1))

## [0.1.1](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/compare/mv64e-rest-to-kafka-gateway-v0.1.0...mv64e-rest-to-kafka-gateway-v0.1.1) (2025-11-20)


### Bug Fixes

* remove unused import ([9019f4c](https://github.com/pcvolkmer/mv64e-rest-to-kafka-gateway/commit/9019f4c780bcee8b08ac35f1dbdd30ae804621a8))
