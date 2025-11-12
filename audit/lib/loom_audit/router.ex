defmodule Loom.Audit.Router do
  use Plug.Router

  plug Plug.Parsers,
    parsers: [:json],
    pass: ["application/json"],
    json_decoder: Jason

  # GraphQL endpoint
  plug :match
  plug :dispatch

  forward "/api",
    to: Absinthe.Plug,
    init_opts: [schema: Loom.Audit.Schema]

  # GraphiQL UI for testing
  forward "/graphiql",
    to: Absinthe.Plug.GraphiQL,
    init_opts: [schema: Loom.Audit.Schema, interface: :simple]

  match _, do: send_resp(conn, 404, "Not Found")
end
