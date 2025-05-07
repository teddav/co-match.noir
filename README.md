# CoMatch 🍑🌶️ Dating

> A private dating experiment built with real cryptography, using MPC and ZK proofs.

## 🧠 What is this?

Co-Match is a privacy-preserving dating prototype with:

🔐 End-to-end encrypted preferences

🤝 Match discovery via secure MPC (thanks to [co-snarks](https://github.com/TaceoLabs/co-snarks/))

✅ Mutual match proof with ZK-SNARKs (thanks to [Noir](https://github.com/noir-lang/noir))

🙈 No swiping. No profile. No public data.

Built during [#NoirHack](https://www.noirhack.com/) as a fun experiment to combine MPC and ZK.

Live Demo (might be slow — ping me if it is!): https://co-match.vercel.app/

## 🧪 How it works

You enter your preferences and your Twitter handle, so your matches can contact you 🌶️.  
Since I have your Twitter, it's not completely private... 😏 Next step: build an in-app chat.

Preferences are encrypted in your browser.  
Ok, I'm lying here... 🙊 [co-noir](https://github.com/TaceoLabs/co-snarks/tree/main/co-noir/co-noir) cannot yet run in the browser (not possible to compile to wasm), so I'm actually encrypting your preferences on the server. But this will soon be changed!

They're sent to multiple MPC servers that check for mutual matches.  
Third time you caught me lying... I didn't want to pay for 3 servers for this PoC, so I'm actually running everything on 1 server, but it's spinning 3 local listeners, so it's kind of the same... 😂

If a match is found: A ZK proof is generated (with Noir) that confirms the match without revealing your preferences.  
If there's no match: no one ever knows.

## Run

### MPC server

You'll need a powerful server to compute the proofs fast. I'm renting an Hetzner's CCX33 (8 vcpus, 32GB) and each proof takes about 450ms to generate.

Run the [config.sh](./mpc-server/config.sh) file to generate the certificates and keys for each server.

You'll need to add an env variable for the JWT token: `JWT_SECRET`

### Front

Edit the [config](./web-app/next.config.ts) with the address of the API
