<p align="center">
  <a href="https://huione.com">
    <img alt="HUIONE" src="https://i.imgur.com/IKyzQ6T.png" width="250" />
  </a>
</p>

# HUIONE Geyser Plugin Interface

This crate enables an plugin to be added into the HUIONE Validator runtime to
take actions at the time of account updates or block and transaction processing;
for example, saving the account state to an external database. The plugin must
implement the `GeyserPlugin` trait. Please see the detail of the
`geyser_plugin_interface.rs` for the interface definition.

The plugin should produce a `cdylib` dynamic library, which must expose a `C`
function `_create_plugin()` that instantiates the implementation of the
interface.

The https://github.com/huione-labs/huione-accountsdb-plugin-postgres repository
provides an example of how to create a plugin which saves the accounts data into
an external PostgreSQL databases.

More information about HUIONE is available in the [HUIONE documentation](https://docs.huione.com/).


