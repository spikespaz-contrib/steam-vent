# Changelog

## 0.6.0 (unreleased)

### Breaking

#### Authentication

- `AuthConfirmationHandler::or` has been removed, use tuples of confirmation
  handlers instead.
- `AuthConfirmationHandler::handle_confirmation` now takes `&mut self` instead
  of `self`.

#### Connection

- The lower level function `one_with_header`, `on_with_header`,
  `send_with_kind`, `raw_send` and `raw_send_with_kind` have been removed from
  the `ConnectionTrait` in order to trim the public api.

#### Messages

- The `EncodableMessage` trait has been split op into `DecodableMessage` and
  `EncodableMessage` traits.
- The `NetMessage` trait has been split op into `SendableMessage` and
  `ReceivableMessage` traits.
- The `ServiceMethodRequest` trait has been split op into `ServiceMethodRequest`
  and `ServiceNotification` traits.

### Changes

- Steam guard confirmation now retries when an incorrect token is provided,
  `AuthConfirmationHandler::handle_confirmation` will be called once each
  attempts.
- The `ConnectionTrait` and related message traits have been moved to
  `steam-vent-common` (and are re-exported from `steam-vent`). This provides a
  smaller api surface for higher level wrappers (e.g. `steam-vent-chat`) to
  allow decoupling their version from `steam-vent` in most cases.
