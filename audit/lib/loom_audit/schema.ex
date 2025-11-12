defmodule Loom.Audit.Schema do
  use Absinthe.Schema

  alias Loom.Audit.Resolvers

  # --- Types ---
  object :ritual_output do
    field :success, :boolean
    field :message, :string
    field :trace_log, :string
    field :ai_suggestion, :string
  end

  # --- Queries ---
  query do
    @desc "Run the full symbolic audit"
    field :run_audit, :ritual_output do
      arg :context, :string
      resolve &Resolvers.run_audit/3
    end
  end
end
