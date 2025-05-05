# Local testing

```bash
npm i -g vercel@latest
vercel build
vercel dev -t "<TOKEN>"
```

# Flow

## frontend

- input preferences
- split into shares
- send each share to a different MPC server
- wait for matching

## backend (mpc server)

- receives shares (both "user1" and "user2")
- matches user1 against all user2
- write matches to DB
- stores user2

# MPC Server

```bash
cargo run --bin gen_cert -- -k data/key0.der -c data/cert0.der -s localhost -s ip6-localhost -s 127.0.0.1 -s party0
```
