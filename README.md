# linku-sona

Types from https://www.npmjs.com/package/@kulupu-linku/sona,
ported to rust. Up to date as of @kulupu-linku/sona v0.2.0

All types derive `serde::Serialize`, `serde::Deserialize` and `std::fmt::Debug`,
so you can use them with any serde-compatible format. All fields are public, so you can access them directly.

Do note that string literals (like in `<Word>.book`) have all been converted to enums (`Book` in
this case) for better type safety, but you can get the original string with `to_string()`.

If something's wrong or missing, please open an issue or a pull request.
