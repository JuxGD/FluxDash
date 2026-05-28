{
  stdenv
, lib
, rustPlatform
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "fluxdash";
  version = "0.3.0";
  src = ./.;
})
