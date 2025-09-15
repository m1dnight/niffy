defmodule NiffyTest do
  use ExUnit.Case
  doctest Niffy

  test "greets the world" do
    assert Niffy.hello() == :world
  end
end
