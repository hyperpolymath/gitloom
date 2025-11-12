defmodule Loom.Audit.Resolvers do
  alias Loom.Audit.AISuggester
  alias Loom.Audit.KanrenGenerator

  def run_audit(_parent, %{context: context}, _resolution) do
    _ = context
    ai_suggestion = AISuggester.suggest("reweaver", true)
    repo_name = "gitloom"

    # --- 1. Generate Kanren Facts (Neuro-Symbolic Setup) ---
    # Call the new Elixir module directly, bypassing the shell.
    KanrenGenerator.generate(repo_name)
    IO.puts("Generated Kanren facts for #{repo_name}.")

    # --- 2. Run Symbolic Logic (Prolog via Just) ---
    cmd = "just"
    args = ["--justfile", "Justfile", "run-prolog"] # Run only the Prolog step
    opts = [cd: "..", stderr_to_stdout: true]

    case System.cmd(cmd, args, opts) do
      {_output, 0} ->
        # Logic pipeline succeeded
        trace_log =
          case File.read("../logic/output.ndjson") do
            {:ok, body} -> body
            {:error, _} -> "Error: Could not read trace log."
          end

        {:ok,
         %{
           success: true,
           message: "Audit passed. Symbolic attestation complete.",
           trace_log: trace_log,
           ai_suggestion: ai_suggestion
         }}

      {output, _exit_code} ->
        # Logic pipeline FAILED
        {:ok,
         %{
           success: false,
           message: "Audit FAILED. Symbolic attestation blocked.",
           trace_log: output,
           ai_suggestion: ai_suggestion
         }}
    end
  end
end
