{ inputs, ... }: {
  perSystem = { config, system, ... }:
    let
      rustFlake =
        inputs.flake-lang.lib.${system}.rustFlake {
          src = ./.;
          crateName = "plutus-ledger-api";
          version = "1.0.0";
          devShellHook = config.settings.shell.hook;
          cargoNextestExtraArgs = "--all-features";
        };
    in
    {
      inherit (rustFlake) packages checks devShells;
    };
}
