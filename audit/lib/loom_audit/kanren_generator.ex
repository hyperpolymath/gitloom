defmodule Loom.Audit.KanrenGenerator do
  @moduledoc """
  Generates the initial Prolog fact corpus, eliminating the shell script dependency.
  """
  
  @doc "Generates Prolog facts based on a simple repository state."
  def generate(repo_name) do
    # This logic was previously in kanren/sandboxed-kanren.sh
    facts = [
      "fixed_head(#{repo_name}).",
      "created_orphan_branch(#{repo_name}).",
      "repo_configured(#{repo_name}).",
      "pipeline_present(#{repo_name})."
    ]

    # Write the facts directly to the file needed by Prolog
    File.write!("../logic/kanren_facts.pl", Enum.join(facts, "\n"))
  end
end
