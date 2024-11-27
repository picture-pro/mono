#[cfg(all(
  not(debug_assertions),
  target_arch = "wasm32",
  not(feature = "hydrate")
))]
compile_error!(
  "You are attempting to compile `site-app` on a `wasm32` target without \
   enabling the `hydrate` feature."
);

#[cfg(all(
  not(debug_assertions),
  feature = "hydrate",
  not(target_arch = "wasm32")
))]
compile_error!(
  "You are attempting to compile `site-app` with the `hydrate` feature on a \
   platform which is not `wasm32`."
);

#[cfg(all(
  not(debug_assertions),
  not(target_arch = "wasm32"),
  not(feature = "ssr")
))]
compile_error!(
  "You are attempting to compile `site-app` on a target which is not `wasm32` \
   without enabling the `ssr` feature."
);

#[cfg(all(not(debug_assertions), feature = "ssr", target_arch = "wasm32"))]
compile_error!(
  "You are attempting to compile `site-app` with the `ssr` feature on a \
   `wasm32` target."
);

#[cfg(all(
  not(debug_assertions),
  not(feature = "hydrate"),
  not(feature = "ssr")
))]
compile_error!(
  "You are attempting to compile `site-app` without enabling the `hydrate` or \
   `ssr` feature."
);

#[cfg(all(not(debug_assertions), feature = "hydrate", feature = "ssr"))]
compile_error!(
  "You are attempting to compile `site-app` with both the `hydrate` and `ssr` \
   features enabled."
);
