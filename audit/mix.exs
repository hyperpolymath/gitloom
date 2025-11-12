defmodule Loom.Audit.MixProject do
  use Mix.Project

  def project do
    [
      app: :loom_audit,
      version: "0.1.0",
      elixir: "~> 1.15",
      elixirc_paths: elixirc_paths(Mix.env()),
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      mod: {Loom.Audit.Application, []},
      extra_applications: [:logger, :runtime_tools]
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  defp deps do
    [
      # Your preferred server
      {:bandit, "~> 1.0"},
      # GraphQL
      {:phoenix, "~> 1.7.0"},
      {:absinthe, "~> 1.7"},
      {:absinthe_plug, "~> 1.5"},
      # JSON library
      {:jason, "~> 1.2"}
    ]
  end
end
