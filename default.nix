{
  stdenv
, lib
, rustPlatform
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "rider";
  version = "0.0.0";
  src = "./src";
})
