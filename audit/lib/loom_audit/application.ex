defmodule Loom.Audit.Application do
  @moduledoc false
  use Application

  @impl true
  def start(_type, _args) do
    # FIX: Calculate the path by going up THREE levels to gitloom/
    certs_root = Path.expand("../../../config/certs", __DIR__)
    keyfile_path = Path.join(certs_root, "self_signed.key")
    certfile_path = Path.join(certs_root, "self_signed.pem")

    children = [
      {Bandit,
       plug: Loom.Audit.Router,
       scheme: :https,
       port: 4000,
       otp_app: :loom_audit,
       # Paths are now absolute, guaranteed to be found
       keyfile: keyfile_path,
       certfile: certfile_path} 
    ]

    opts = [strategy: :one_for_one, name: Loom.Audit.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
