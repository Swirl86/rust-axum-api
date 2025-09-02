# Starta backend i nytt fönster
Start-Process powershell -ArgumentList "cd 'g:\Programmering\Workspace\rust-axum-api\rusty-cart\backend'; cargo run"

# Starta frontend i nytt fönster
Start-Process powershell -ArgumentList "cd 'g:\Programmering\Workspace\rust-axum-api\rusty-cart\frontend'; trunk serve"