defmodule Niffy.NIF do
  use Rustler,
    otp_app: :niffy,
    crate: :niffy

  # When loading a NIF module, dummy clauses for all NIF function are required.
  # NIF dummies usually just error out when called when the NIF is not loaded, as that should never normally happen.
  def test(), do: :erlang.nif_error(:nif_not_loaded)

  def test_back(_), do: :erlang.nif_error(:nif_not_loaded)
end
