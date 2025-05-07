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

If a match is found: A ZK proof is generated (with Noir) that confirms the match without revealing your preferences.  
If there's no match: no one ever knows.

## Run

### MPC server

### Front

```bash
npm i -g vercel@latest
vercel build
vercel dev -t "<TOKEN>"
```
