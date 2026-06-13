# Changelog

## 0.6.0 (unreleased)

### Breaking

- `AuthConfirmationHandler::or` has been removed, use tuples of confirmation handlers instead.
- `AuthConfirmationHandler::handle_confirmation` now takes `&mut self` instead of `self`

### Changes

- Steam guard confirmation now retries when an incorrect token is provided, `AuthConfirmationHandler::handle_confirmation` will be called once each attempts.
