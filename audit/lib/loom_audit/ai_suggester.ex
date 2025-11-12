defmodule Loom.Audit.AISuggester do
  @moduledoc "Replaces loom-shadow.lfe. Provides AI-style suggestions."

  def suggest(role, care_mode) do
    case {role, care_mode} do
      {"reweaver", true} ->
        "🧠 Suggest: Emit badge overlay with hearts."
      {"auditor", false} ->
        "⚠️ Suggest: Lock down overlay and run deep audit."
      {"ritualist", true} ->
        "💖 Suggest: Add emotional resonance to the .adoc log."
      _ ->
        "🤖 Suggest: Unknown role/care_mode. Initiate introspection."
    end
  end
end
